use std::collections::HashMap;
use std::sync::LazyLock;

use hfst::transducer_actor::HfstTransducerActor;
use without_ats::without_ats;

use crate::pipelines::PipelineError;
use crate::pipelines::gather_consecutive_equals;
use crate::pipelines::get_langfile_generator;

#[derive(serde::Serialize)]
pub struct GenerateResult {
    /// The analysis string that was generated (i.g. `lemma+pos+tag1+tag2+...`)
    pub analysis: String,
    /// All wordforms generated from this analysis string.
    pub wordforms: Vec<String>,
}

impl std::fmt::Display for GenerateResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for wordform in self.wordforms.iter() {
            write!(f, "{}\t", self.analysis)?;
            for part in without_ats::without_ats_iter(wordform) {
                write!(f, "{part}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::convert::From<(String, Vec<String>)> for GenerateResult {
    fn from((analysis, wordforms): (String, Vec<String>)) -> Self {
        GenerateResult {
            analysis,
            wordforms,
        }
    }
}

pub async fn generate_subprocess(lang: &str, input: &str) -> Result<String, PipelineError> {
    let input = input.to_owned();
    let generator = get_langfile_generator(lang)?;

    tokio::task::spawn_blocking(
        move || cmd_lib::run_fun!(echo "$input" | hfst-lookup -q $generator),
    )
    .await?
    .map_err(PipelineError::from)
}

/// The result is a Vec, which each element corresponding to the input.
/// Each element is a 2-tuple (input, results), where `input` is the same string
/// as in the inputs, and results is a Vec of all results, the actual result string,
/// and the weight.
pub async fn generate_libhfst(
    lang: &str,
    input: &str,
) -> Result<Vec<GenerateResult>, PipelineError> {
    let Some(actor) = HFST_TRANSDUCER_ACTORS.get(lang) else {
        return Err(PipelineError::missing_generator_hfstol(lang));
    };

    let inputs = input
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    let mut out = vec![];
    for input in inputs {
        let it = actor
            .lookup(input)
            .await?
            .results
            .into_iter()
            .map(|(form, _weight)| (input, without_ats(&form)));
        out.extend(crate::pipelines::gather(it, GenerateResult::from));
    }
    Ok(out)
}

pub fn parse_generate_subprocess_results(s: &str) -> Vec<GenerateResult> {
    let it = s
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut it = line.split('\t');
            let analysis = it.next()?;
            let generated_form = it.next()?;
            let _weight = it.next()?;
            assert_eq!(it.next(), None);
            Some((analysis, generated_form))
        });
    gather_consecutive_equals(it)
        .into_iter()
        .map(GenerateResult::from)
        .collect()
}
type ActorMap = HashMap<String, hfst::transducer_actor::HfstTransducerActor>;

pub static HFST_TRANSDUCER_ACTORS: LazyLock<ActorMap> = LazyLock::new(|| {
    use crate::langmodel_files::{LANGMODEL_FILES, LANGS, WP_LANGFOLDER};
    let langfolder = std::path::PathBuf::from(&*WP_LANGFOLDER);
    LANGS
        .iter()
        .filter_map(|lang| {
            LANGMODEL_FILES
                .files
                .iter()
                .filter(|file| file.filename == "generator-gt-norm.hfstol")
                .filter_map(|file| file.find_on_system(&langfolder, lang))
                .map(|(path, _)| hfst::HfstInputStream::new(&path).expect("tocttou never happens"))
                .map(|input_stream| {
                    input_stream
                        .read_only_transducer()
                        .expect("generator-gt-norm.hfstol has exactly 1 transducer")
                })
                .map(|transducer| {
                    HfstTransducerActor::builder()
                        .transducer(transducer)
                        .queue_size(std::num::NonZeroUsize::new(100).unwrap())
                        .build()
                })
                .map(|actor| (lang.to_string(), actor))
                .next()
        })
        .collect()
});
