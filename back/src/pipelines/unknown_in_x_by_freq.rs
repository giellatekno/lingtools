use crate::langmodel_files::get_langfile;
use crate::pipelines::PipelineError;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;
use tracing::trace;

const NOB_ALPHABET: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'æ', 'ø', 'å', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'Æ', 'Ø',
    'Å',
];

// extracts lines like
//   "liten" A Pos Fem Sg Indef <W:0.0>
//   "setning" N Fem Sg Indef <W:0.0>
//   "giwjlehge" ?
//   "?" CLB <W:0.0>
//   "og" CC <W:0.0>
//   """ PUNCT <W:0.0>
// into a tuple of the word inside the first parens, and the
// part after it (the analysis)
fn extract_word_and_analysis(line: &str) -> (&str, &str) {
    let splits = line.split('"').collect::<Vec<_>>();
    let nsplits = splits.len();
    if nsplits == 4 {
        // a literal " (as in: """ PUNCT <W:0.0>)
        ("\"", &splits[3][1..])
    } else if nsplits == 3 {
        // not a literal "
        (splits[1], &splits[2][1..])
    } else {
        // sometimes there is more, I don't know what it is, just return
        // the the a '"' as the word, it will be filtered out
        ("\"", "")
    }
}

// unwanted analyses: "?", and PUNCT and CLB
fn unwanted_analyses((_word, analysis): &(&str, &str)) -> bool {
    analysis != &"?" || !analysis.contains("PUNCT") || !analysis.contains("CLB")
}

// unwanted words: words that do not contain any norwegian letters
fn unwanted_words((word, _analysis): &(&str, &str)) -> bool {
    word.contains(NOB_ALPHABET)
}

pub async fn unknown_in_x_by_freq_subprocess(
    input: String,
    lang1: String,
    lang2: String,
) -> Result<String, PipelineError> {
    let Some(tokdisamb) = get_langfile(&lang1, "tokeniser-disamb-gt-desc.pmhfst") else {
        return Err(PipelineError::missing_tokenizer_pmhfst(&lang1));
    };
    let disambcg = get_langfile(&lang1, "disambiguator.cg3")
        .or_else(|| get_langfile(&lang1, "disambiguator.bin"))
        .ok_or_else(|| {
            let file = "disambiguator.(cg3|bin)".to_string();
            PipelineError::missing_files(&lang1, Some(vec![file]))
        })?;

    let dicthfst = format!("{}{}all.hfst", lang1, lang2);
    let Some(dict) = get_langfile(&lang1, &dicthfst) else {
        return Err(PipelineError::missing_files(&lang1, Some(vec![dicthfst])));
    };

    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path().to_path_buf();
    temp_file
        .as_file()
        .write_all(input.as_bytes())
        .map_err(PipelineError::from)?;

    //let cut_delim = "-d\"";
    let results = tokio::task::spawn_blocking(move || {
        cmd_lib::run_fun!(
            cat $path |
            hfst-tokenise -cg $tokdisamb |
            vislcg3 -g $disambcg |
            grep -v "^[:\"]"// |
            //cut $cut_delim -f2 |
            //uniq |
            //sort |
            //uniq -c |
            //sort -nr |
            //cut -c9- |
            //grep -v "[0-9A-ZÆØÅ]" |
            //grep "[a-zæøå]" |
            //lookup $dict
        )
    })
    .await?
    .map_err(PipelineError::from)?;

    let recognized_words = results
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(extract_word_and_analysis)
        .filter(unwanted_analyses)
        .filter(|(word, _analysis)| word.contains(NOB_ALPHABET))
        .map(|(word, _analysis)| word)
        .collect::<Vec<_>>()
        .join("\n");

    let temp_file2 = NamedTempFile::new().map_err(PipelineError::from)?;
    let path = temp_file2.path();
    temp_file2
        .as_file()
        .write_all(recognized_words.as_bytes())
        .map_err(PipelineError::from)?;

    trace!(recognized_words);

    /*
    let lookup_binary = std::env::var("LOOKUP_BINARY").unwrap_or_else(|_| String::from("lookup"));
    let lookup_results = run_fun!(
        cat $path |
        $lookup_binary $dict
    )
    .map_err(|e| e.to_string())?;
    */
    //let cmd_str = format!("hfst-lookup -q --input={path:?}

    let mut cmd = std::process::Command::new("hfst-lookup");
    cmd.arg("-q");
    cmd.arg(&dict);
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    match std::fs::exists(path) {
        Ok(true) => {}
        Ok(false) => {
            panic!("file {path:?} verified to not exist!");
        }
        Err(e) => {
            panic!("file {path:?} could not be determined if it exists or not! Error: {e}");
        }
    }

    let mut child_handle = cmd
        .spawn()
        .inspect_err(|error| {
            let command = format!("{cmd:?}");
            tracing::warn!(?command, ?error, "could not run command");
        })
        .map_err(PipelineError::from)?;

    child_handle
        .stdin
        .as_mut()
        .unwrap()
        .write_all(recognized_words.as_bytes())
        .map_err(PipelineError::from)?;

    let lookup_results = match child_handle.wait_with_output() {
        Ok(output) => {
            if output.status.code().expect("child output has status code") == 0 {
                String::from_utf8(output.stdout).expect("stdout from hfst-lookup is valid utf-8")
            } else {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let command = format!("{cmd:?}");
                tracing::warn!(?stdout, ?stderr, command, "non-0 output from cmd");
                let err = PipelineError::SubprocessError {
                    source: std::io::Error::other("non-0 from command"),
                };
                return Err(err);
            }
        }
        Err(error) => {
            let command = format!("{cmd:?}");
            tracing::warn!(?error, command, "could not run command");
            let err = PipelineError::SubprocessError {
                source: std::io::Error::other("could not run command"),
            };
            return Err(err);
        }
    };

    // counts of how many times unrecognized word appears
    let mut counts: HashMap<&str, usize> = HashMap::new();

    lookup_results
        .lines()
        // only interested in words that were not found in the dictionary
        // those lines are "<word>\t<word>\t?" - so filter out lines that
        // does not end with ?, and extract the first word
        .filter_map(|line| line.split('\t').collect_tuple())
        .filter(|(_input, output, _weight)| output.ends_with("+?"))
        .for_each(|(word, _output, _weight)| {
            counts
                .entry(word)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    // collect the pairs into a vec
    let mut counts: Vec<(&str, usize)> =
        counts.iter().map(|(&word, &count)| (word, count)).collect();

    // ...so that we can sort them by the count..
    counts.sort_by_cached_key(|(_word, count)| *count);
    // biggest first
    counts.reverse();

    let some_counts = counts
        .iter()
        .map(|(k, v)| format!("k={k}, v={v}"))
        .join("; ");
    trace!(some_counts, "after sorting the counts");

    // finally output them as <count>\t<word>  one per line
    let final_results = counts
        .iter()
        .map(|(word, count)| format!("{}\t{}", count, word))
        .collect::<Vec<_>>()
        .join("\n");

    trace!(final_results, "final results to be sent");

    Ok(final_results)
}
