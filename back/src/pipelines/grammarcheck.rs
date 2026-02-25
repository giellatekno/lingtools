//! Grammarcheck, equalvalent to what is provided by by divvun-runtime

use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Response},
};

use crate::common_url::{Format, FormatQueryParam, LangAndInputParams};

pub async fn grammarcheck_endpoint(
    Path(LangAndInputParams { lang, input }): Path<LangAndInputParams>,
    format: Query<FormatQueryParam>,
) -> Response {
    let format = format.unwrap_or(Format::Text);

    let result = match match tokio::task::spawn_blocking(move || {
        crate::grammarcheck::grammarcheck(&lang, &input)
    })
    .await
    {
        Ok(r) => r,
        Err(e) => return format!("grammarcheck() panicd: {}", e).into_response(),
    } {
        Ok(res) => res,
        Err(e) => return e.into_responder(format).into_response(),
    };

    match format {
        Format::Text => result.into_response(),
        Format::Json | Format::PrettyJson => {
            let obj = serde_json::json!({
                "results": result,
            });
            axum::Json(obj).into_response()
        }
    }
}
