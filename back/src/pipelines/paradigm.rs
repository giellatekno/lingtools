use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use once_cell::sync::Lazy;
use serde::Serialize;
use tokio::sync::RwLock;
use tokio::{fs::File, io::AsyncReadExt};

use crate::pipelines::{PipelineError, gather};
use crate::pipelines::generate::{GenerateResult, parse_generate_subprocess_results};

use crate::pipelines::analyze::{AnalysisResult, parse_analyse_subprocess_results};
use crate::pipelines::analyze::analyze_libhfst;

use crate::pipelines::analyze::analyze_subprocess;
use crate::pipelines::generate::generate_libhfst;
use crate::pipelines::generate::generate_subprocess;
use analysis_string_parser::{AnalysisParts, Pos, parse_analysis_parts};

use crate::paradigm::ParadigmSize;

// (lang, size) => Result<paradigm_file (string), error message string>
static PARADIGM_FILES: Lazy<RwLock<HashMap<(String, ParadigmSize), Result<String, String>>>> =
    Lazy::new(|| RwLock::new(HashMap::with_capacity(16)));

#[derive(Serialize)]
pub struct ParadigmOutput {
    /// The input.
    pub input: (String, Option<Pos>),
    /// All the generated forms.
    pub generated_forms: Vec<GenerateResult>,
    /// A list of other analyses for which the input lemma was a conjugated form
    /// of another word.
    pub other_forms: Vec<AnalysisParts>,
}

impl std::fmt::Display for ParadigmOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.generated_forms.len() == 0 {
            writeln!(f, "[no results]")?;
        } else {
            for result in self.generated_forms.iter() {
                writeln!(f, "{result}")?;
            }
        };
        writeln!(f)?;
        write!(f, "'{}' is ", self.input.0)?;
        if self.other_forms.len() == 0 {
            writeln!(f, "not a conjugated form of any other lemmas")?;
        } else {
            writeln!(f, "also a conjugated form of:")?;
            for form in self.other_forms.iter() {
                writeln!(f, "- {form}")?;
            }
        };
        Ok(())
    }
}

/*
#[derive(Debug, thiserror::Error)]
pub enum AllParadigmsError {
    #[error("paradigm definition file error: {0}")]
    ParadigmDefintionError(String),
    #[error("generator error: {0}")]
    Generate(#[from] GenerateLibhfstError),
}

/// Generate paradigms, using the libhfst generator. The output item is the analysis
/// string that produced the item, as well as the item itself. If the `pos` is `None`,
/// it means 'any'.
///
/// ```not_rust
/// Output: [
///     ["lemma+pos+A+B+C+D", [ "form1", "form2", ... ],
///     ["lemma+pos+A+B+F+G", [ "form1", "form2", ... ],
///     ...
/// ]
/// ```
async fn all_paradigms_libhfst(
    input_lemma: &str,
    lang: &str,
    pos: Option<Pos>,
    size: ParadigmSize,
) -> Result<Vec<GenerateResult>, AllParadigmsError> {
    let paradigm_file = get_paradigmfile(lang, size)
        .await
        .map_err(|e| AllParadigmsError::ParadigmDefintionError(e))?;

    let all_potential_forms = get_potential_forms(
        &lang, size, pos, &input_lemma, &paradigm_file);
    Ok(generate_libhfst(&all_potential_forms, lang)
        .await?)
}
*/

pub async fn paradigm_libhfst(
    lang: &str,
    input: &str,
    wanted_pos: Option<Pos>,
    size: ParadigmSize,
) -> Result<ParadigmOutput, PipelineError> {
    let mut seen = HashSet::new();
    let mut generated_forms: Vec<GenerateResult> = vec![];
    let mut other_forms = vec![];

    let analyses = analyze_libhfst(lang, input).await?;

    for AnalysisResult { wordform, analyses } in analyses {
        for analysis in analyses {
            let analysis = without_ats::without_ats(&analysis);
            let Some(analysed) = parse_analysis_parts(&analysis, "+") else {
                tracing::trace!(analysis, "raw analysis line failed to parse");
                continue;
            };
            let analysis_pos = analysed
                .pos
                .expect("every analysis string we get back from analysis has a pos");

            // Filter so that we only show the pos that was wanted. If wanted_pos is
            // None, then we don't filter out anything, because user wants all.
            if let Some(wanted_pos) = wanted_pos
                && wanted_pos != analysis_pos
            {
                continue;
            }

            let anl_lemma = analysed.lemma().expect("all analyses have a lemma");
            let key = (anl_lemma.to_owned(), analysed.pos);
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);

            if anl_lemma != input {
                other_forms.push(analysed);
                continue;
            }

            // Find all paradigms of this (lemma, pos)
            // this is what `all_paradigm_forms` also does
            let paradigm_file = get_paradigmfile(lang, size)
                .await
                .map_err(|e| PipelineError::ParadigmFile(e))?;
            let all_potential_forms = get_potential_forms(input, wanted_pos, &paradigm_file);

            generated_forms = generate_libhfst(lang, &all_potential_forms).await?;
        }
    }

    let input = (input.to_owned(), wanted_pos);
    Ok(ParadigmOutput {
        input,
        generated_forms,
        other_forms,
    })
}

pub async fn paradigm_subprocess(
    lang: &str,
    input: &str,
    wanted_pos: Option<Pos>,
    size: ParadigmSize,
) -> Result<ParadigmOutput, PipelineError> {
    let analyses = analyze_subprocess(lang, input, false).await?;
    let analyses = parse_analyse_subprocess_results(&analyses);

    let mut seen = HashSet::new();
    let mut generated_forms: Vec<GenerateResult> = vec![];
    let mut other_forms = vec![];
    for AnalysisResult { wordform, analyses } in analyses {
        for analysis in analyses {
            let analysis = parse_analysis_parts(&analysis, "+")
                .expect("analysis from analysis is not empty");
            let pos = analysis.pos.expect("analysis from analysis has a pos");

            // if wanted_pos is None, that means any, so will never hit. If a specific
            // pos is wanted, continue if the analysis doesn't have that pos
            if let Some(wanted_pos) = wanted_pos && wanted_pos != pos {
                continue;
            }

            let lemma = analysis.lemma().expect("all analyses have lemmas");

            if seen.contains(&(lemma.clone(), pos)) {
                // this (lemma, pos) has already been covered
                continue;
            } else {
                seen.insert((lemma.clone(), pos));
            }

            if lemma != input {
                other_forms.push(analysis);
                continue;
            }

            let paradigm_file = get_paradigmfile(lang, size)
                .await
                .map_err(|e| PipelineError::ParadigmFile(e))?;
            let all_potential_forms = get_potential_forms(input, Some(pos), &paradigm_file);
            let generated = generate_subprocess(lang, &all_potential_forms).await?;
            let vec = parse_generate_subprocess_results(&generated);
            generated_forms.extend(vec);
        }
    }

    let input = (input.to_owned(), wanted_pos);
    Ok(ParadigmOutput { input, generated_forms, other_forms })
}

/// Generate a newline delimited `String` of the `input_lemma+tags`, where `tags`
/// are all the potential forms that can have a generated form, for the given language,
/// and paradigm size. Used as input to the generator when finding all paradigms.
fn get_potential_forms(lemma: &str, pos: Option<Pos>, paradigm_file: &str) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    paradigm_file
        .lines()
        .map(|line| {
            parse_analysis_parts(line, "+").expect("line in paradigm file parses as an analysis")
        })
        .filter(|para| {
            match (pos, para.pos) {
                (Some(x), Some(y)) => x == y,
                (Some(_x), None) => panic!("paradigm file line has no pos?"),
                // We're asking for all poses
                (None, _) => true,
            }
        })
        .for_each(|analysis_parts| {
            writeln!(out, "{lemma}+{analysis_parts}").expect("write! to String is ok");
        });
    out
}

/// Get the paradigm file for a 2-tuple of (lang, size) from the cache. The error
/// variant of the Result is a `String`, because it's easier to store for later,
/// than a `std::io::Error`.
async fn get_paradigmfile(lang: &str, size: ParadigmSize) -> Result<String, String> {
    let key = (lang.to_owned(), size);
    let guard = PARADIGM_FILES.read().await;
    match guard.get(&key) {
        Some(paradigm_file) => paradigm_file.clone(),
        None => {
            drop(guard);
            let paradigm_file = generate_paradigm_file(lang, size)
                .await
                .map_err(|e| e.to_string());
            let mut guard = PARADIGM_FILES.write().await;
            guard.insert(key, paradigm_file.clone());
            paradigm_file
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GenerateParadigmFileError {
    #[error("missing definition file: {0}")]
    MissingDefinitionFile(String),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Generate the file of all paradigms
async fn generate_paradigm_file(
    lang: &str,
    size: ParadigmSize,
) -> Result<String, GenerateParadigmFileError> {
    use crate::langmodel_files::get_langfile;

    let paradigm_text_file = format!("paradigm_{size}.{lang}.txt");
    let korpustags_file = format!("korpustags.{lang}.txt");
    let gramfile = get_langfile(&lang, &paradigm_text_file)
        .ok_or_else(|| GenerateParadigmFileError::MissingDefinitionFile(paradigm_text_file))?;
    let tagfile = get_langfile(&lang, &korpustags_file)
        .ok_or_else(|| GenerateParadigmFileError::MissingDefinitionFile(korpustags_file))?;

    let gram_entries = read_gramfile(gramfile).await?;
    let tagmap = read_tagfile(tagfile).await?;

    Ok(expand_gram_entries(gram_entries, tagmap))
}

fn expand_gram_entries(
    // N+Stemtype?+Case+...
    grammar_entries: Vec<String>,
    // Stemtype = [A, B, C] ...
    tagmap: HashMap<String, Vec<String>>,
) -> String {
    let mut out = vec![];

    for entry in grammar_entries {
        let splits = entry.split('+');
        let classes = splits.map(|mut split| {
            let mut v = vec![];

            if let Some(stripped) = split.strip_suffix('?') {
                v.push("".to_owned());
                split = stripped;
            }

            match tagmap.get(split) {
                Some(s) => v.extend(s.iter().map(|s| s.clone())),
                None => v.push(split.to_owned()),
            };

            v
        });

        out.extend(
            classes
                .into_iter()
                .multi_cartesian_product()
                .map(|tags_vec| {
                    let tags_s = tags_vec.iter().filter(|s| s.len() > 0).join("+");
                    format!("{tags_s}")
                }),
        );
    }

    out.join("\n")
}

async fn read_gramfile(gramfile: std::path::PathBuf) -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open(gramfile).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    Ok(contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .filter(|line| !line.starts_with(&['#', '%', '$']))
        .map(|line| line.to_string())
        .collect::<Vec<String>>())
}

/// Read and parse the tagfile (a file named "korpustags.txt"),
/// given as a PathBuf to the full path in the `tagfile` argument.
/// Return a hashmap of "tag class" to list of tags, for example:
///   "Wordclass" => ["N", "A", "V", "Adv", ...]
///   "Person-Number" => ["Sg1", "Sg2", "Sg3", "Du1", ...]
///   "Transitivity" => ["TV", "IV"]
///   "Infinite" => ["Inf", "PrfPrc", "PrsPrc", "Sup", "VGen", ...]
async fn read_tagfile(
    tagfile: std::path::PathBuf,
) -> Result<HashMap<String, Vec<String>>, std::io::Error> {
    let mut file = File::open(tagfile).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_vec = vec![];

    contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with(&['%', '$']))
        .filter(|line| !line.contains('='))
        .for_each(|line| {
            if line.starts_with('#') {
                m.insert(line[1..].to_owned(), current_vec.clone());
                current_vec.clear();
            } else {
                // slice up to the first tab, or space, or if there is no
                // tab or space, then until the end of line
                let i = line.find(['\t', ' ']).unwrap_or(line.len());
                let word = line[0..i].to_owned();
                current_vec.push(word);
            }
        });

    Ok(m)
}
