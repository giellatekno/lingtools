mod analysis;
mod async_watcher;
mod common_url;
mod endpoints;
mod file_watcher;
mod generate;
mod grammarcheck;
mod langmodel_files;
mod memmem_split;
mod multimap;
mod paradigm;
mod pipelines;
mod timing;
mod util;
mod weight;

use std::{path::Path, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    extract::DefaultBodyLimit,
    response::{IntoResponse, Response},
    routing::{get, post},
    BoxError, Router,
};
use listenfd::ListenFd;
use tokio::{self, net::TcpListener};
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::CorsLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::async_watcher::make_async_watcher;
use crate::langmodel_files::WP_LANGFOLDER;
use crate::pipelines::lemma_count::lemma_count_endpoint;
use crate::timing::timing_middleware;

async fn handle_error(err: BoxError) -> Response {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            http::StatusCode::REQUEST_TIMEOUT,
            "Handling the request took too long",
        )
            .into_response()
    } else if err.is::<tower::load_shed::error::Overloaded>() {
        (http::StatusCode::SERVICE_UNAVAILABLE, "service is busy").into_response()
    } else {
        (http::StatusCode::INTERNAL_SERVER_ERROR, "unhandled error").into_response()
    }
}

fn handle_panic(
    err: Box<dyn std::any::Any + Send + 'static>,
) -> http::Response<http_body_util::Full<bytes::Bytes>> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };

    let body = serde_json::json!({
        "error": {
            "kind": "panic",
            "details": details,
        }
    });
    let body = serde_json::to_string(&body).unwrap();

    Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(http_body_util::Full::from(body))
        .unwrap()
}

#[tokio::main]
async fn main() {
    info!("starting lingtools api");
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=trace,tower_http=debug,axum::rejection=trace,tokio=debug,runtime=debug",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // it's a Lazy OnceCell, which will exit() if the env var is not set when
    // reading it the first time (which is also part of the reason why we
    // print it out here - to actually read it, to run the check and exit
    // if it was not found..probably a better way to do this)
    info!("Language folder: {}", &*WP_LANGFOLDER);

    // populate the hashmap of known files by reading the file system
    langmodel_files::load_langfiles();
    let n_langfiles = langmodel_files::LANGFILES
        .read()
        .expect("LANGFILES is not poisoned")
        .len();
    info!("Found {n_langfiles} language model files");

    // Set up the file watcher that watches the WP_LANGFOLDER for changes
    let (mut debouncer, rx) = make_async_watcher(Duration::from_secs(1)).unwrap();
    let path = std::path::Path::new(&*WP_LANGFOLDER);
    debouncer
        .watch(path, notify::RecursiveMode::Recursive)
        .unwrap();

    // Set up the handlers that react to the file watcher events
    let _filewatcher_jh = file_watcher::file_watcher()
        .create_fn(Box::new(|path: &Path| {
            langmodel_files::add_langfile(path);
        }))
        .remove_fn(Box::new(|path| {
            langmodel_files::remove_langfile(path);
        }))
        .build()
        .spawn(rx)
        .await;

    let app = Router::new()
        .route(
            "/unknown-lemmas-in-dict",
            post(endpoints::unknown_in_x_by_freq).layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(handle_error))
                    .load_shed()
                    .concurrency_limit(2), //.timeout(Duration::from_secs(2))
            ),
        )
        .route(
            "/lemma-count",
            post(lemma_count_endpoint).layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(handle_error))
                    .timeout(Duration::from_secs(60)),
            ),
        )
        .route("/analyze/{lang}/{input}", get(endpoints::analyze))
        .route("/dependency/{lang}/{input}", get(endpoints::dependency))
        .route("/disambiguate/{lang}/{input}", get(endpoints::disambiguate))
        .route("/generate/{lang}/{input}", get(endpoints::generate))
        .route("/hyphenate/{lang}/{input}", get(endpoints::hyphenate))
        .route("/numbers/{lang}/{input}", get(endpoints::numbers))
        .route("/paradigm/{lang}/{input}", get(endpoints::paradigm))
        .route("/transcribe/{lang}/{input}", get(endpoints::transcribe))
        .route("/info", get(langmodel_files::info_endpoint))
        .layer(axum::middleware::from_fn(timing_middleware))
        // global concurrency limit (applies to all paths)
        // does NOT have load_shed, so will queue requests (on the internal, hidden queue)
        .layer(ConcurrencyLimitLayer::new(200))
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(false))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .layer(ServiceBuilder::new().layer(CorsLayer::very_permissive()))
        // We'll just let the revproxy decide when request bodies are too big
        .layer(DefaultBodyLimit::disable())
        .fallback_service(ServeDir::new("assets"));

    let mut listenfd = ListenFd::from_env();

    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            let port = listener.local_addr().unwrap().port();
            listener.set_nonblocking(true).unwrap();
            let ret = TcpListener::from_std(listener).unwrap();
            info!("Listening on port {port} (via listenfd)");
            ret
        }
        // otherwise fall back to local listening
        None => {
            let ret = TcpListener::bind("0.0.0.0:3000").await.unwrap();
            info!("Listening on port 3000");
            ret
        }
    };

    axum::serve(listener, app).await.unwrap();
}
