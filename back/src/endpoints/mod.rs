use axum::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::response::Response;

use crate::common_url::Format;
use crate::common_url::FormatQueryParam;
use crate::common_url::LangAndInputParams;
use crate::common_url::LibhfstQueryParam;

use crate::pipelines::PipelineError;
use crate::pipelines::analyze::analyze_libhfst;
use crate::pipelines::analyze::analyze_subprocess;
use crate::pipelines::dependency::dependency_libhfst;
use crate::pipelines::dependency::dependency_subprocess;
use crate::pipelines::disambiguate::disambiguate_libhfst;
use crate::pipelines::disambiguate::disambiguate_subprocess;
use crate::pipelines::generate::generate_libhfst;
use crate::pipelines::generate::generate_subprocess;
use crate::pipelines::hyphenate::hyphenate_libhfst;
use crate::pipelines::hyphenate::hyphenate_subprocess;
use crate::pipelines::numbers::numbers_libhfst;
use crate::pipelines::numbers::numbers_subprocess;
use crate::pipelines::paradigm::paradigm_libhfst;
use crate::pipelines::paradigm::paradigm_subprocess;
use crate::pipelines::transcribe::transcribe_libhfst;
use crate::pipelines::transcribe::transcribe_subprocess;
use crate::pipelines::unknown_in_x_by_freq::unknown_in_x_by_freq_subprocess;

enum Responder {
    String(String),
    Json(axum::Json<serde_json::Value>),
}

impl IntoResponse for Responder {
    fn into_response(self) -> Response {
        match self {
            Self::String(inner) => inner.into_response(),
            Self::Json(inner) => inner.into_response(),
        }
    }
}

trait ToResponder {
    fn to_responder(&self, format: Format) -> Responder;
}

impl<T> ToResponder for Result<T, PipelineError>
where
    T: std::fmt::Display + serde::Serialize,
{
    fn to_responder(&self, format: Format) -> Responder {
        match (self, format) {
            (Ok(res), Format::Text) => Responder::String(res.to_string()),
            (Ok(results), Format::Json | Format::PrettyJson) => {
                Responder::Json(axum::Json(serde_json::json!(results)))
            }
            (Err(e), Format::Text) => Responder::String(format!("Error: {e}")),
            (Err(e), Format::Json | Format::PrettyJson) => {
                Responder::Json(axum::Json(serde_json::json!({ "error": e.to_string() })))
            }
        }
    }
}

/// A wrapper type, that endpoints map the Result<_, PipelineError> type into, so that
/// we can implement `IntoResponder` for it. We can't implement `IntoReponder` for
/// `Result`, because `IntoResponder` requires `Display` (so a responder can output
/// "text" formatted results. We can't implement `Display` for `Result` (because we
/// don't own any of them, and the orphan rule prevents us from implementing a trait
/// we don't control on a type we don't control; the orphan rules requires us to
/// control at least one of those two, and so, we create a wrapper type around the
/// result, so that we can implement `Display` for it.
#[derive(Debug, serde::Serialize)]
pub struct StructuredOutput<T> {
    results: T,
}

#[derive(Debug, serde::Serialize)]
pub struct MultiOutput<T> {
    results: T,
}

impl<T> std::fmt::Display for StructuredOutput<Vec<T>>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.results.iter() {
            println!("{item}");
            //tracing::trace!(?item, "item");
            writeln!(f, "{item}")?;
        }
        Ok(())
    }
}

impl<T> std::fmt::Display for MultiOutput<Vec<Vec<T>>>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for outer in self.results.iter() {
            for item in outer {
                writeln!(f, "{item}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// TODO: These pipelines are almost entirely the same, and many of them literally are.
// However, it turned out to not be super trivially easy to just macro them, but it
// should be possible, I think...

pub async fn analyze(
    Path(LangAndInputParams { lang, input }): Path<LangAndInputParams>,
    Query(format): Query<FormatQueryParam>,
    libhfst: Query<LibhfstQueryParam>,
) -> Response {
    let format = format.unwrap_or(Format::Json);
    let use_libhfst = libhfst.unwrap_or_false();

    if use_libhfst {
        analyze_libhfst(&lang, &input)
            .await
            .map(|vec| StructuredOutput { results: vec })
            .to_responder(format)
            .into_response()
    } else {
        // TODO: tokenize? third parameter, the bool, set to false for now
        analyze_subprocess(&lang, &input, false)
            .await
            .map(|s| crate::pipelines::analyze::parse_analyse_subprocess_results(&s))
            .map(|vec| StructuredOutput { results: vec })
            .to_responder(format)
            .into_response()
    }
}

pub async fn numbers(
    Path(LangAndInputParams { lang, input }): Path<LangAndInputParams>,
    Query(format): Query<FormatQueryParam>,
    libhfst: Query<LibhfstQueryParam>,
    Query(numbers_method): Query<crate::pipelines::numbers::NumbersPipeline>,
) -> Response {
    let format = format.unwrap_or(Format::Json);
    let use_libhfst = libhfst.unwrap_or_false();

    if use_libhfst {
        numbers_libhfst(&lang, &input, numbers_method)
            .await
            //.map(|vec| StructuredOutput { results: vec })
            .to_responder(format)
            .into_response()
    } else {
        numbers_subprocess(&lang, &input, numbers_method)
            .await
            .to_responder(format)
            .into_response()
    }
}

pub async fn hyphenate(
    Path(LangAndInputParams { lang, input }): Path<LangAndInputParams>,
    Query(format): Query<FormatQueryParam>,
    libhfst: Query<LibhfstQueryParam>,
) -> Response {
    let format = format.unwrap_or(Format::Json);
    let use_libhfst = libhfst.unwrap_or_false();

    if use_libhfst {
        hyphenate_libhfst(&lang, &input)
            .await
            .map(|vec| StructuredOutput { results: vec })
            .to_responder(format)
            .into_response()
    } else {
        hyphenate_subprocess(&lang, &input)
            .await
            .to_responder(format)
            .into_response()
    }
}

pub async fn generate(
    Path(LangAndInputParams { lang, input }): Path<LangAndInputParams>,
    Query(format): Query<FormatQueryParam>,
    libhfst: Query<LibhfstQueryParam>,
) -> Response {
    let format = format.unwrap_or(Format::Json);
    let use_libhfst = libhfst.unwrap_or_false();

    if use_libhfst {
        generate_libhfst(&lang, &input)
            .await
            .map(|vec| StructuredOutput { results: vec })
            .to_responder(format)
            .into_response()
    } else {
        generate_subprocess(&lang, &input)
            .await
            .map(|s| crate::pipelines::generate::parse_generate_subprocess_results(&s))
            .map(|vec| StructuredOutput { results: vec })
            .to_responder(format)
            .into_response()
    }
}

pub async fn dependency(
    Path(LangAndInputParams { lang, input }): Path<LangAndInputParams>,
    Query(format): Query<FormatQueryParam>,
    libhfst: Query<LibhfstQueryParam>,
) -> Response {
    let format = format.unwrap_or(Format::Json);
    let use_libhfst = libhfst.unwrap_or_false();

    if use_libhfst {
        dependency_libhfst(&lang, &input)
            .await
            .to_responder(format)
            .into_response()
    } else {
        dependency_subprocess(&lang, &input)
            .await
            .to_responder(format)
            .into_response()
    }
}

pub async fn disambiguate(
    Path(LangAndInputParams { lang, input }): Path<LangAndInputParams>,
    Query(format): Query<FormatQueryParam>,
    libhfst: Query<LibhfstQueryParam>,
) -> Response {
    let format = format.unwrap_or(Format::Json);
    let use_libhfst = libhfst.unwrap_or_false();

    if use_libhfst {
        disambiguate_libhfst(&lang, &input)
            .await
            .to_responder(format)
            .into_response()
    } else {
        disambiguate_subprocess(&lang, &input)
            .await
            .to_responder(format)
            .into_response()
    }
}

pub async fn transcribe(
    Path(LangAndInputParams { lang, input }): Path<LangAndInputParams>,
    Query(format): Query<FormatQueryParam>,
    libhfst: Query<LibhfstQueryParam>,
) -> Response {
    let format = format.unwrap_or(Format::Json);
    let use_libhfst = libhfst.unwrap_or_false();

    if use_libhfst {
        transcribe_libhfst(&lang, &input)
            .await
            .to_responder(format)
            .into_response()
    } else {
        transcribe_subprocess(&lang, &input)
            .await
            .to_responder(format)
            .into_response()
    }
}

#[derive(serde::Deserialize)]
pub struct SizeAndPosParams {
    size: Option<crate::paradigm::ParadigmSize>,
    pos: Option<crate::paradigm::AcceptedPos>,
}

pub async fn paradigm(
    Path(LangAndInputParams { lang, input }): Path<LangAndInputParams>,
    Query(SizeAndPosParams { size, pos }): Query<SizeAndPosParams>,
    Query(format): Query<FormatQueryParam>,
    libhfst: Query<LibhfstQueryParam>,
) -> Response {
    let wanted_pos = pos
        .unwrap_or(crate::paradigm::AcceptedPos::Any)
        .to_standard_pos();
    let use_libhfst = libhfst.unwrap_or_false();
    let size = size.unwrap_or(crate::paradigm::ParadigmSize::Standard);
    let format = format.unwrap_or(Format::Text);

    if use_libhfst {
        paradigm_libhfst(&lang, &input, wanted_pos, size)
            .await
            .to_responder(format)
            .into_response()
    } else {
        paradigm_subprocess(&lang, &input, wanted_pos, size)
            .await
            .to_responder(format)
            .into_response()
    }
}

#[derive(serde::Deserialize)]
pub struct UnknownInXByFreqInput {
    typ: String,
    lang1: String,
    lang2: String,
    data: String,
}

pub async fn unknown_in_x_by_freq(
    Query(format): Query<FormatQueryParam>,
    Json(body): Json<UnknownInXByFreqInput>,
) -> Response {
    use crate::util::{gunzip, read_docx_text};
    use base64::{Engine as _, engine::general_purpose};
    use http::StatusCode;

    const UE: StatusCode = http::StatusCode::UNPROCESSABLE_ENTITY;

    let format = format.unwrap_or(Format::Json);
    let lang1 = body.lang1;
    let lang2 = body.lang2;

    let text: String = match body.typ.as_str() {
        "text" => body.data,
        "text+gz+b64" => {
            let Ok(gz_data) = general_purpose::STANDARD.decode(body.data) else {
                return (UE, "could not base64 decode data").into_response();
            };

            let Ok(text_data) = gunzip(gz_data) else {
                return (UE, "failed to gunzip data").into_response();
            };

            let Ok(text) = String::from_utf8(text_data) else {
                return (UE, "text not valid utf-8").into_response();
            };
            text
        }
        "docx" => {
            let Ok(decoded) = general_purpose::STANDARD.decode(body.data) else {
                return (UE, "could not base64 decode data").into_response();
            };

            let Some(text) = read_docx_text(decoded) else {
                return (UE, "could not read docx file").into_response();
            };
            text
        }
        _ => return (UE, "'typ' field must be text, text+gz+b64 or docx").into_response(),
    };

    unknown_in_x_by_freq_subprocess(text, lang1, lang2)
        .await
        .to_responder(format)
        .into_response()
}
