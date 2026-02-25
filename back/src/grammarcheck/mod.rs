use axum::response::IntoResponse;
use cmd_lib::run_fun;

use crate::common_url::Format;

#[derive(Debug, thiserror::Error)]
pub enum GrammarcheckError {
    #[error("Language not supported: {0}")]
    LangNotSupported(String),
    /// Pipeline Error
    #[error("pipeline error: {0}")]
    PipelineError(String),
}

pub struct GrammarcheckErrorResponder {
    err: GrammarcheckError,
    format: Format,
}

impl GrammarcheckError {
    pub fn into_responder(self, format: Format) -> GrammarcheckErrorResponder {
        GrammarcheckErrorResponder { err: self, format }
    }
}

impl IntoResponse for GrammarcheckErrorResponder {
    fn into_response(self) -> axum::response::Response {
        let err = format!("{}", self.err);
        match self.format {
            Format::Json | Format::PrettyJson => axum::Json(serde_json::json!({
                "error": err,
            }))
            .into_response(),
            Format::Text => err.into_response(),
        }
    }
}

/// Run the full grammarchecker pipeline.
///
/// ```typescript
/// export default function smeGramRelease(entry: StringEntry): Command {
///   let x = hfst.tokenize("tokenize", entry, { model_path: "tokeniser-gramcheck-gt-desc.pmhfst" });
///   x = divvun.blanktag("whitespace", x, { model_path: "analyser-gt-whitespace.hfst" });
///   x = cg3.vislcg3("valency", x, { model_path: "valency.bin" });
///   x = cg3.vislcg3("mwe-dis", x, { model_path: "mwe-dis.bin" });
///   x = cg3.mwesplit("mwesplit", x);
///   x = divvun.blanktag("errorwhitespace", x, { model_path: "analyser-gt-errorwhitespace.hfst" });
///   x = divvun.cgspell("speller", x, {
///     acc_model_path: "acceptor.default.hfst",
///     err_model_path: "errmodel.default.hfst",
///   });
///   x = cg3.vislcg3("postspell-valency", x, { model_path: "valency-postspell.bin" });
///   x = cg3.vislcg3("grc-disamb", x, { model_path: "grc-disambiguator.bin" });
///   x = cg3.vislcg3("spell-sugg-filtering", x, { model_path: "spellchecker.bin" });
///   x = cg3.vislcg3("gramcheck", x, { model_path: "grammarchecker-release.bin" });
///   return divvun.suggest("suggestions", x, { model_path: "generator-gramcheck-gt-norm.hfstol" });
/// }
/// ```
pub fn grammarcheck(input: &str, _lang: &str) -> Result<String, GrammarcheckError> {
    use GrammarcheckError as Error;

    let input = input.to_owned();

    run_fun!(
        echo $input |
        hfst-lookup -q nonexistant.hfstol
    )
    .map_err(|e| Error::PipelineError(e.to_string()))
}

pub async fn grammarcheck_async(input: String, lang: String) -> Result<String, GrammarcheckError> {
    tokio::task::spawn_blocking(move || grammarcheck(&input, &lang))
        .await
        .expect("task did not panic")
}
