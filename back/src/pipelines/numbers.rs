use crate::pipelines::PipelineError;
//use crate::pipelines::get_langfile_numbers;
use crate::pipelines::get_langfile;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NumbersDirection {
    Digit2Text,
    Text2Digit,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NumbersMethod {
    Numbers,
    Clock,
    Date,
    Year,
}

#[derive(serde::Deserialize)]
pub struct NumbersPipeline {
    method: NumbersMethod,
    direction: NumbersDirection,
}

pub async fn numbers_libhfst(
    _lang: &str,
    _input: &str,
    _pipeline: NumbersPipeline,
) -> Result<String, PipelineError> {
    unimplemented!()
}

pub async fn numbers_subprocess(
    lang: &str,
    input: &str,
    NumbersPipeline { method, direction }: NumbersPipeline,
) -> Result<String, PipelineError> {
    let input = input.to_owned();
    let langfile = match (method, direction) {
        (NumbersMethod::Clock, NumbersDirection::Digit2Text) => {
            "transcriptor-clock-digit2text.filtered.lookup.hfstol"
        }
        (NumbersMethod::Date, NumbersDirection::Digit2Text) => {
            "transcriptor-date-digit2text.filtered.lookup.hfstol"
        }
        (NumbersMethod::Numbers, NumbersDirection::Digit2Text) => {
            "transcriptor-numbers-digit2text.filtered.lookup.hfstol"
        }
        (NumbersMethod::Year, NumbersDirection::Digit2Text) => {
            "transcriptor-year-digit2text.filtered.lookup.hfstol"
        }
        (NumbersMethod::Clock, NumbersDirection::Text2Digit) => {
            "transcriptor-clock-text2digit.filtered.lookup.hfstol"
        }
        (NumbersMethod::Date, NumbersDirection::Text2Digit) => {
            "transcriptor-date-text2digit.filtered.lookup.hfstol"
        }
        (NumbersMethod::Numbers, NumbersDirection::Text2Digit) => {
            "transcriptor-numbers-text2digit.filtered.lookup.hfstol"
        }
        (NumbersMethod::Year, NumbersDirection::Text2Digit) => {
            "transcriptor-year-text2digit.filtered.lookup.hfstol"
        }
    };
    tracing::trace!(lang, langfile, "call get_langfile()");
    let hfstol = get_langfile(lang, langfile).ok_or_else(|| {
        tracing::trace!("no langfile!!");
        PipelineError::missing_files(lang, Some(vec![langfile.to_string()]))
    })?;
    tracing::trace!("got langfile");

    tokio::task::spawn_blocking(move || cmd_lib::run_fun!(echo "$input" | hfst-lookup -q $hfstol))
        .await?
        .map_err(PipelineError::from)
}
