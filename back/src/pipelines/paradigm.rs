use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use once_cell::sync::Lazy;
use serde::Serialize;
use tokio::sync::RwLock;
use tokio::{fs::File, io::AsyncReadExt};

use crate::pipelines::PipelineError;
use crate::pipelines::generate::{GenerateResult, parse_generate_subprocess_results};

use crate::pipelines::analyze::analyze_libhfst;
use crate::pipelines::analyze::analyze_subprocess;
use crate::pipelines::analyze::{AnalysisResult, parse_analyse_subprocess_results};
use crate::pipelines::generate::generate_libhfst;
use crate::pipelines::generate::generate_subprocess;
use analysis_string_parser::{AnalysisParts, Pos, parse_analysis_parts};

use crate::paradigm::ParadigmSize;

// (lang, size) => Result<paradigm_file (string), error message string>
static PARADIGM_FILES: Lazy<RwLock<HashMap<(String, ParadigmSize), Result<String, String>>>> =
    Lazy::new(|| RwLock::new(HashMap::with_capacity(16)));

#[derive(Serialize)]
pub struct Form {
    /// The rest of the tags, "+Sg+Nom",
    pub tags: String,
    /// Forms of the word,
    pub forms: Vec<String>,
}

#[derive(Serialize)]
pub struct ParadigmForm {
    pub lemma: String,
    pub pos: Pos,
    pub subclass: Option<String>,
    pub forms: Vec<Form>,
}

impl std::fmt::Display for ParadigmForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for Form { tags, forms } in self.forms.iter() {
            for form in forms.iter() {
                write!(f, "{}+{}", self.lemma, self.pos)?;
                if let Some(ref subclass) = self.subclass {
                    let _ = write!(f, "+{subclass}");
                }
                write!(f, "+{tags}\t{form}\n")?;
            }
        }
        Ok(())
    }
}


#[derive(Serialize)]
pub struct OtherForm {
    pub lemma: String,
    pub pos: Pos,
    pub subclass: Option<String>,
}

#[derive(Serialize)]
pub struct ParadigmOutput {
    /// The input.
    pub input: (String, Option<Pos>),
    /// All the generated forms.
    pub paradigm_forms: Vec<ParadigmForm>,
    /// A list of other analyses for which the input lemma was a conjugated form
    /// of another word.
    pub other_forms: Vec<OtherForm>,
}

impl std::fmt::Display for ParadigmOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.paradigm_forms.len() == 0 {
            writeln!(f, "[no results]")?;
        } else {
            for result in self.paradigm_forms.iter() {
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
                //writeln!(f, "- {form}")?;
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
    let mut paradigm_forms: HashMap<(String, Pos, Option<String>), Vec<Form>> = HashMap::new();

    let mut other_forms = vec![];

    for AnalysisResult { wordform, analyses } in analyze_libhfst(lang, input).await? {
        for analysis in analyses {
            let analysis = without_ats::without_ats(&analysis);

            let Some(analysed) = parse_analysis_parts(&analysis) else {
                tracing::trace!(analysis, "raw analysis line failed to parse");
                continue;
            };

            let Some(analysis_pos) = analysed.pos else {
                tracing::error!("analysis that came back from analysis had no pos");
                continue;
            };

            if let Some(wanted_pos) = wanted_pos
                && wanted_pos != analysis_pos
            {
                continue;
            }

            let Some(anl_lemma) = analysed.lemma() else {
                tracing::error!("analysis has no lemma ??");
                continue;
            };

            let key = (anl_lemma.to_owned(), analysed.pos);
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);

            if anl_lemma != input {
                let mut subclass = None;
                for part in analysed.parts.iter() {
                    if let Some(tag) = part.tag() && tag.is_subclass() {
                        subclass = Some(tag.to_string());
                    }
                }

                other_forms.push(OtherForm {
                    lemma: anl_lemma,
                    pos: analysed.pos.unwrap(),
                    subclass,
                });

                continue;
            }

            let paradigm_file = get_paradigmfile(lang, size)
                .await
                .map_err(|e| PipelineError::ParadigmFile(e))?;

            let prefix = analysed.generation_string_prefix();
            let all_potential_forms = get_potential_forms(&prefix, wanted_pos, &paradigm_file);

            let generated_forms = generate_libhfst(lang, &all_potential_forms).await?;
            for GenerateResult { analysis, wordforms } in generated_forms {
                let analysis = parse_analysis_parts(&analysis)
                    .expect("analysis from generator is not empty");
                let lemma = analysis.lemma().expect("has a lemma");
                let pos = analysis.pos.expect("has a pos");
                let mut subclass = None;
                for part in analysis.parts.iter() {
                    if let Some(tag) = part.tag() && tag.is_subclass() {
                        subclass = Some(tag.to_string());
                    }
                }

                let entry = paradigm_forms.entry((lemma, pos, subclass)).or_default();

                use std::fmt::Write;
                use analysis_string_parser::OwnedTag;
                let mut tags = String::new();
                analysis.parts
                    .iter()
                    .filter(|&part| {
                        match part.tag() {
                            Some(OwnedTag::Pos(_)) => false,
                            Some(tag) if tag.is_subclass() => false,
                            Some(OwnedTag::Cmp) => false,
                            Some(OwnedTag::CmpX(_)) => false,
                            Some(_other_tag) => true,
                            None => false,
                        }
                    })
                    .for_each(|part| {
                        let _ = write!(tags, "{part}+");
                    });

                // Some words have only Pos, no tags!
                if tags.ends_with("+") {
                    tags.pop();
                }

                entry.push(Form { tags, forms: wordforms });
            }
        }
    }

    let input = (input.to_owned(), wanted_pos);

    let paradigm_forms: Vec<ParadigmForm> = paradigm_forms.into_iter()
        .map(|((lemma, pos, subclass), forms)| ParadigmForm {
            lemma,
            pos,
            subclass,
            forms,
        })
        .collect();

    Ok(ParadigmOutput {
        input,
        paradigm_forms,
        other_forms,
    })
}

pub async fn paradigm_subprocess(
    lang: &str,
    input: &str,
    wanted_pos: Option<Pos>,
    size: ParadigmSize,
) -> Result<ParadigmOutput, PipelineError> {
    let analyses = analyze_subprocess(lang, input, true).await?;
    let analyses = parse_analyse_subprocess_results(&analyses);
    println!("{analyses:?}");

    let mut seen = HashSet::new();
    let mut paradigm_forms: HashMap<(String, Pos, Option<String>), Vec<Form>> = HashMap::new();
    let mut other_forms = vec![];
    for AnalysisResult { wordform, analyses } in analyses {
        for analysis in analyses {
            let Some(analysed) = parse_analysis_parts(&analysis) else {
                tracing::trace!(analysis, "raw analysis failed to parse");
                continue;
            };

            let Some(analysis_pos) = analysed.pos else {
                tracing::error!("analysis that came back from analysis had no pos");
                continue;
            };

            if let Some(wanted_pos) = wanted_pos
                && wanted_pos != analysis_pos
            {
                continue;
            }

            let Some(anl_lemma) = analysed.lemma() else {
                tracing::error!("analysis has no lemma ??");
                continue;
            };

            let key = (anl_lemma.to_owned(), analysed.pos);
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);

            if anl_lemma != input {
                let mut subclass = None;
                for part in analysed.parts.iter() {
                    if let Some(tag) = part.tag() && tag.is_subclass() {
                        subclass = Some(tag.to_string());
                    }
                }

                other_forms.push(OtherForm {
                    lemma: anl_lemma,
                    pos: analysed.pos.unwrap(),
                    subclass,
                });

                continue;
            }

            let paradigm_file = get_paradigmfile(lang, size)
                .await
                .map_err(|e| PipelineError::ParadigmFile(e))?;
            let prefix = analysed.generation_string_prefix();
            let all_potential_forms = get_potential_forms(&prefix, wanted_pos, &paradigm_file);
            let generated_forms = generate_subprocess(lang, &all_potential_forms).await?;
            let generated_forms = parse_generate_subprocess_results(&generated_forms);

            for GenerateResult { analysis, wordforms } in generated_forms {
                let analysis = parse_analysis_parts(&analysis)
                    .expect("analysis from generator is not empty");
                let lemma = analysis.lemma().expect("has a lemma");
                let pos = analysis.pos.expect("has a pos");
                let mut subclass = None;
                for part in analysis.parts.iter() {
                    if let Some(tag) = part.tag() && tag.is_subclass() {
                        subclass = Some(tag.to_string());
                    }
                }

                let entry = paradigm_forms.entry((lemma, pos, subclass)).or_default();

                let tags = analysis.parts
                    .iter()
                    .filter(|&part| {
                        match part.tag() {
                            Some(tag) => !tag.is_subclass() && !tag.is_pos(),
                            None => false,
                        }
                    })
                    .map(|part| format!("{part}"))
                    .intersperse(String::from("+"))
                    .collect::<String>();

                entry.push(Form { tags, forms: wordforms });
            }
        }
    }

    let input = (input.to_owned(), wanted_pos);

    let paradigm_forms: Vec<ParadigmForm> = paradigm_forms.into_iter().map(|((lemma, pos, subclass), forms)| ParadigmForm {
        lemma, pos, subclass, forms
    })
    .collect();
    Ok(ParadigmOutput {
        input,
        paradigm_forms,
        other_forms,
    })
}

/// Generate a newline delimited `String` of the `input_lemma+tags`, where `tags`
/// are all the potential forms that can have a generated form, for the given language,
/// and paradigm size. Used as input to the generator when finding all paradigms.
fn get_potential_forms(prefix: &str, pos: Option<Pos>, paradigm_file: &str) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    paradigm_file
        .lines()
        .map(|line| {
            parse_analysis_parts(line).expect("line in paradigm file parses as an analysis")
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
            let _ = writeln!(out, "{prefix}+{analysis_parts}");
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
