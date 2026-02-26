use std::collections::HashMap;
use std::sync::LazyLock;

use without_ats::without_ats;

use crate::langmodel_files::get_langfile;
use crate::pipelines::{PipelineError, gather_consecutive_equals, get_langfile_tokenizer};

#[derive(Debug, serde::Serialize)]
pub struct AnalysisResult {
    /// e.g. "hus"
    pub wordform: String,
    /// e.g. `["hus+N+Neu+Pl+Indef", "hus+N+Neu+Sg+Indef", "huse+V+Imp"]`
    /// ```
    pub analyses: Vec<String>,
}

impl std::convert::From<(String, Vec<String>)> for AnalysisResult {
    fn from((wordform, analyses): (String, Vec<String>)) -> Self {
        AnalysisResult { wordform, analyses }
    }
}

impl std::fmt::Display for AnalysisResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for analysis in self.analyses.iter() {
            writeln!(f, "{}\t{analysis}", self.wordform)?;
        }
        Ok(())
    }
}

pub fn parse_analyse_subprocess_results(s: &str) -> Vec<AnalysisResult> {
    let it = s
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut it = line.split('\t');
            let wordform = it.next()?;
            let analysis = it.next()?;
            let _weight = it.next()?;
            assert_eq!(
                it.next(),
                None,
                "never more than 3 fields in analysis output"
            );
            Some((wordform, analysis))
        });

    gather_consecutive_equals(it)
        .into_iter()
        .map(AnalysisResult::from)
        .collect()
}

/// Analyse the `input` with the analyser for language `lang`. The `tokenize` argument
/// determines if the input is run through the tokenizer before analysis, or not.
pub async fn analyze_subprocess(
    lang: &str,
    input: &str,
    tokenize: bool,
) -> Result<String, PipelineError> {
    let Some(analyser) = get_langfile(lang, "analyser-gt-desc.hfstol") else {
        return Err(PipelineError::missing_analyser_hfstol(lang));
    };

    let input = input.to_owned();
    if tokenize {
        let tokenizer = get_langfile_tokenizer(lang)?;

        tokio::task::spawn_blocking(move || {
            cmd_lib::run_fun!(
                echo "$input" |
                hfst-tokenize -q $tokenizer |
                hfst-lookup -q --beam=0 $analyser
            )
        })
        .await?
    } else {
        tokio::task::spawn_blocking(
            move || cmd_lib::run_fun!(echo "$input" | hfst-lookup -q --beam=0 $analyser),
        )
        .await?
    }
    .map_err(PipelineError::from)
}

pub async fn analyze_libhfst(
    lang: &str,
    input: &str,
) -> Result<Vec<AnalysisResult>, PipelineError> {
    let Some(actor) = HFST_TRANSDUCER_ACTORS.get(lang) else {
        return Err(PipelineError::missing_analyser_hfstol(lang));
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
            .map(|(item, _weight)| (input, without_ats(&item)));
        out.extend(crate::pipelines::gather(it, AnalysisResult::from));
    }
    Ok(out)
}

/// The hfst transducer actors, one actor for each language. The file is always
/// "analyser-gt-desc.hfstol".
type ActorMap = HashMap<String, hfst::transducer_actor::HfstTransducerActor>;

static HFST_TRANSDUCER_ACTORS: LazyLock<ActorMap> = LazyLock::new(|| {
    use crate::langmodel_files::{LANGMODEL_FILES, LANGS, WP_LANGFOLDER};
    let langfolder = std::path::PathBuf::from(&*WP_LANGFOLDER);

    LANGS
        .iter()
        .filter_map(|lang| {
            LANGMODEL_FILES
                .files
                .iter()
                .filter(|file| file.filename == "analyser-gt-desc.hfstol")
                .filter_map(|file| file.find_on_system(&langfolder, lang))
                .map(|(path, _)| {
                    hfst::HfstInputStream::new(&path).expect("tocttou never happens to us")
                })
                .map(|input_stream| {
                    input_stream
                        .read_only_transducer()
                        .expect("analyser-gt-desc.hfstol has exactly 1 transducer")
                })
                .map(|transducer| {
                    hfst::transducer_actor::HfstTransducerActor::builder()
                        .transducer(transducer)
                        .queue_size(std::num::NonZeroUsize::new(100).unwrap())
                        .build()
                })
                .map(|actor| (lang.to_string(), actor))
                .next()
        })
        .collect()
});
