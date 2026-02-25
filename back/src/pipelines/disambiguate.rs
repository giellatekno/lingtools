use crate::pipelines::PipelineError;
use crate::pipelines::get_langfile_disambiguator;
use crate::pipelines::get_langfile_tokenizer;

pub async fn disambiguate_subprocess(lang: &str, input: &str) -> Result<String, PipelineError> {
    let tokenizer = get_langfile_tokenizer(lang)?;
    let disamb = get_langfile_disambiguator(lang)?;
    let input = input.to_owned();

    tokio::task::spawn_blocking(move || {
        cmd_lib::run_fun!(
            echo "$input" |
            hfst-tokenize -cg $tokenizer |
            vislcg3 -g $disamb
        )
    })
    .await?
    .map_err(PipelineError::from)
}

pub async fn disambiguate_libhfst(_lang: &str, _input: &str) -> Result<String, PipelineError> {
    // TODO implement when hfst-rs can do hfst-tokenize
    unimplemented!("requires that hfst-rs can do hfst-tokenize, also no vislcg3...");
}
