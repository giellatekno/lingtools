use crate::pipelines::PipelineError;
use crate::pipelines::get_langfile_hyphenator;

pub async fn hyphenate_subprocess(lang: &str, input: &str) -> Result<String, PipelineError> {
    let hyphenator = get_langfile_hyphenator(lang)?;
    let input = input.replace(" ", "\n");

    tokio::task::spawn_blocking(
        move || cmd_lib::run_fun!(echo "$input" | hfst-lookup -q $hyphenator),
    )
    .await?
    .map_err(PipelineError::from)
}

pub async fn hyphenate_libhfst(_lang: &str, _input: &str) -> Result<Vec<String>, PipelineError> {
    unimplemented!("hyphenate libhfst")
}
