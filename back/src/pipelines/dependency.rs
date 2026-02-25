use crate::pipelines::PipelineError;
use crate::pipelines::get_langfile_dependency;
use crate::pipelines::get_langfile_disambiguator;
use crate::pipelines::get_langfile_korp;
use crate::pipelines::get_langfile_tokenizer;

pub async fn dependency_subprocess(lang: &str, input: &str) -> Result<String, PipelineError> {
    let tokenizer = get_langfile_tokenizer(lang)?;
    let disambiguator = get_langfile_disambiguator(lang)?;
    let korp = get_langfile_korp(lang)?;
    let dependency = get_langfile_dependency(lang)?;

    let use_weight = match lang {
        "rus" => true,
        //"nob" => true,
        _ => false,
    };

    let tokenize_params = if use_weight { "-cg" } else { "-cgW" };

    let input = input.to_owned();
    tokio::task::spawn_blocking(move || {
        cmd_lib::run_fun!(
            echo "$input" |
            hfst-tokenize $tokenize_params $tokenizer |
            vislcg3 -g $disambiguator |
            vislcg3 -g $korp |
            vislcg3 -g $dependency
        )
    })
    .await?
    .map_err(PipelineError::from)
}

pub async fn dependency_libhfst(_lang: &str, _input: &str) -> Result<String, PipelineError> {
    unimplemented!("dependency can't make use of libhfst, need to implmenent at least tokenize");
}
