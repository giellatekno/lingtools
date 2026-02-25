use crate::langmodel_files::get_langfile;
use crate::pipelines::PipelineError;

pub async fn transcribe_subprocess(lang: &str, input: &str) -> Result<String, PipelineError> {
    let file = "txt2ipa.lookup.hfstol";
    let Some(txt2ipa) = get_langfile(&lang, file) else {
        return Err(PipelineError::missing_files(
            lang,
            Some(vec![file.to_string()]),
        ));
    };

    let input = input.to_owned();
    tokio::task::spawn_blocking(move || cmd_lib::run_fun!(echo "$input" | hfst-lookup -q $txt2ipa))
        .await?
        .map_err(PipelineError::from)
}

pub async fn transcribe_libhfst(_lang: &str, _input: &str) -> Result<String, PipelineError> {
    unimplemented!()
}
