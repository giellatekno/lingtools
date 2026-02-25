//! pipelines
//!
//! This mod contains the pipelines, how they are run, and how they are converted to
//! axum responses.
//!
//! They are called pipelines, because traditionally, they are shell pipelines. You echo
//! some text, pipe it through numerous instances of `hfst-tokenize`, `vislcg3`, etc,
//! and get out the desired responses. They still run as pipelines, using the `cmd_lib`
//! library, but some are also implemented with the ability to use binddings to the
//! underlying libraries directly. For now, some hfst functionality is implemented, via
//! `hfst-rs`, but most is missing, and for pipeline-functionality requiring cg3, nothing
//! is implemented (yet?).
//!
//! The submods here correspond to axum endpoints. The functions and structures are often
//! structured the same way:
//!   `async fn endpoint` - the axum endpoint
//!   `X_subprocess` - the pipeline implemented as a "shell pipeline", returning a String
//!   `X_libhfst` - the pipeline functionality implemented via libs, returns structured
//!   `response_from_string` - turns string response from `X_subprocess` to a response
//!   `response_from_structured` - turns structured response into axum response
//!
pub mod analyze;
pub mod dependency;
pub mod disambiguate;
pub mod generate;
pub mod grammarcheck;
pub mod hyphenate;
pub mod lemma_count;
pub mod numbers;
pub mod paradigm;
pub mod transcribe;
pub mod unknown_in_x_by_freq;

use crate::langmodel_files::get_langfile;

/// Common error type for what goes wrong with a pipeline
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    /// Language not supported
    #[error("language '{lang}' not supported")]
    LangNotSupported {
        /// Language code
        lang: String,
        /// If known, Some(vector) of which files are missing, or `None` if
        /// it is not known which files are missing.
        missing_files: Option<Vec<String>>,
    },
    /// A subprocess-based pipeline error.
    /// This error only originates with the subprocess-based pipelines.
    #[error("subprocess error: {}", .source)]
    SubprocessError {
        // cmd_lib::run_fun!() returns a std::io::Result
        source: std::io::Error,
    },
    /// This error only originates with the tokio actor-based pipeline runner
    #[error("actor lookup error: {}", .source)]
    ActorError {
        #[from]
        source: hfst::transducer_actor::LookupError,
    },
    #[error("paradigm file error: {0}")]
    ParadigmFile(String),
}

impl From<std::io::Error> for PipelineError {
    fn from(source: std::io::Error) -> Self {
        Self::SubprocessError { source }
    }
}

impl From<tokio::task::JoinError> for PipelineError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::SubprocessError {
            source: std::io::Error::other(value.to_string()),
        }
    }
}

pub fn get_langfile_hyphenator(lang: &str) -> Result<std::path::PathBuf, PipelineError> {
    let file = "hyphenator-gt-desc.hfstol";
    get_langfile(lang, file)
        .ok_or_else(|| PipelineError::missing_files(lang, Some(vec![String::from(file)])))
}

impl PipelineError {
    /// Create a new `LangNotSupported`
    pub fn missing_files(lang: &str, missing_files: Option<Vec<String>>) -> Self {
        Self::LangNotSupported {
            lang: lang.to_string(),
            missing_files,
        }
    }

    /// Convenience function
    pub fn missing_tokenizer_pmhfst(lang: &str) -> Self {
        Self::LangNotSupported {
            lang: lang.to_string(),
            missing_files: Some(vec!["tokenizer-disamb-gt-desc.pmhfst".to_string()]),
        }
    }

    /// Convenience function
    pub fn missing_analyser_hfstol(lang: &str) -> Self {
        Self::LangNotSupported {
            lang: lang.to_string(),
            missing_files: Some(vec!["analyser-gt-desc.hfstol".to_string()]),
        }
    }

    /// Convenience function
    pub fn missing_generator_hfstol(lang: &str) -> Self {
        Self::LangNotSupported {
            lang: lang.to_string(),
            missing_files: Some(vec!["generator-gt-norm.hfstol".to_string()]),
        }
    }
}

// TODO change the name of this
pub fn gather<OutputType>(
    it: impl Iterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    output_map_fn: impl Fn((String, Vec<String>)) -> OutputType,
) -> Vec<OutputType> {
    let mut map = std::collections::HashMap::<String, Vec<String>>::new();
    it.for_each(|(key, item)| {
        map.entry(key.as_ref().to_string())
            .and_modify(|vec| vec.push(item.as_ref().to_string()))
            .or_insert_with(|| vec![item.as_ref().to_string()]);
    });
    map.drain().map(output_map_fn).collect()
}

pub fn get_langfile_generator(lang: &str) -> Result<std::path::PathBuf, PipelineError> {
    get_langfile(lang, "generator-gt-norm.hfstol")
        .ok_or(PipelineError::missing_generator_hfstol(lang))
}

pub fn get_langfile_korp(lang: &str) -> Result<std::path::PathBuf, PipelineError> {
    get_langfile(lang, "korp.cg3")
        .or_else(|| get_langfile(&lang, "functions.bin"))
        .ok_or_else(|| {
            let file = "(korp.cg3|functions.bin)".to_string();
            PipelineError::missing_files(lang, Some(vec![file]))
        })
}

pub fn get_langfile_tokenizer(lang: &str) -> Result<std::path::PathBuf, PipelineError> {
    get_langfile(lang, "tokeniser-disamb-gt-desc.pmhfst")
        .ok_or(PipelineError::missing_tokenizer_pmhfst(lang))
}

pub fn get_langfile_disambiguator(lang: &str) -> Result<std::path::PathBuf, PipelineError> {
    get_langfile(lang, "disambiguator.cg3")
        .or_else(|| get_langfile(&lang, "disambiguator.bin"))
        .ok_or_else(|| {
            let file = "disambiguator.(cg3|bin)".to_string();
            PipelineError::missing_files(lang, Some(vec![file]))
        })
}

pub fn get_langfile_dependency(lang: &str) -> Result<std::path::PathBuf, PipelineError> {
    get_langfile(lang, "dependency.cg3")
        .or_else(|| get_langfile(&lang, "dependency.bin"))
        .ok_or_else(|| {
            let file = "dependency.(cg3|bin)".to_string();
            PipelineError::missing_files(lang, Some(vec![file]))
        })
}
