// use std::time::Instant;
// use std::{collections::HashMap, ops::Range};
//
// use crate::langmodel_files::get_langfile;
// use cmd_lib::run_fun;
// use tracing::trace;
//
// #[inline]
// fn eat_until(s: &str, start: usize, ch: char) -> Result<Range<usize>, ()> {
//     Ok(start..start + s[start..].find(ch).ok_or(())?)
// }
//
// // std::str::pattern::Pattern isn't stable, so eat_until() above can't take
// // a Pattern.
// // Ideally I'd be able to tell the compiler that the output char must be
// // one of the chars given in `chars`, for pattern matching to not consider
// // the match non-exhaustive if we don't capture the _
// fn eat_until_one_of(s: &str, start: usize, chars: &[char]) -> Result<(Range<usize>, char), ()> {
//     for &ch in chars.iter() {
//         match eat_until(s, start, ch) {
//             Ok(range) => return Ok((range, ch)),
//             Err(_) => continue,
//         }
//     }
//     Err(())
// }
//
// /// A wrapped analysis line String, with helpers to access the inner
// /// fields.
// // implemented as owning the String, and fields storing the ranges of the
// // fields into that string. Accessor methods returns a slice of the `line`,
// // indexed by the stored range.
// // It was easier than storing the String and keeping &str references to itself.
// #[derive(Debug)]
// pub struct Analysis {
//     pub line: String,
//     wordform_range: Range<usize>,
//     lemma_range: Range<usize>,
//     pos_range: Range<usize>,
//     tags_range: Range<usize>,
// }
//
// impl std::str::FromStr for Analysis {
//     type Err = ();
//
//     /// Try to parse an analysis line, as it comes from the analysis,
//     /// that is, strings of this format:
//     /// "<input_word> \t <lemma> "+" <pos> "+" <...remaining tags> \t <weight>"
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let line = s.to_string();
//         let mut i: usize = 0;
//
//         let wordform_range = eat_until(&line, i, '\t')?;
//         i += wordform_range.len() + 1;
//         let lemma_range = eat_until(&line, i, '+')?;
//         i += lemma_range.len() + 1;
//
//         let pos_or_hom_range = match eat_until_one_of(&line, i, &['+', '\t']) {
//             Ok((r, '+')) => r,
//             Ok((r, '\t')) => {
//                 // If we found a \t here, that means input was in form
//                 // lemma+pos \t weight
//                 // so we're done already
//                 return Ok(Self {
//                     line,
//                     wordform_range,
//                     lemma_range,
//                     pos_range: r,
//                     tags_range: 0..0, // no additional tags, so just make an empty string
//                 });
//             }
//             // we only match on + and \t, so we can't get any other char
//             Ok((_r, _)) => unreachable!(),
//             Err(_) => return Err(()),
//         };
//
//         // sometimes, the second entry is not pos, but "Hom1", as in
//         // (smj) båråluluv bårråt+Hom1+V+IV+Cond+Sg1
//         i += pos_or_hom_range.len() + 1;
//
//         let pos_or_hom = line.get(pos_or_hom_range.clone()).unwrap();
//         let pos_range = match pos_or_hom {
//             "Hom1" | "Hom2" | "Hom3" => {
//                 // in case of Hom, we need another eat until + to read pos
//                 let pos_range = eat_until(&line, i, '+')?;
//                 i += pos_range.len() + 1;
//                 pos_range
//             }
//             // sme "fertet" has "v1" in front of the pos!
//             "v1" | "v2" => {
//                 let pos_range = eat_until(&line, i, '+')?;
//                 i += pos_range.len() + 1;
//                 pos_range
//             }
//             _ => pos_or_hom_range,
//         };
//
//         let tags_range = eat_until(&line, i, '\t')?;
//
//         Ok(Self {
//             line,
//             wordform_range,
//             lemma_range,
//             pos_range,
//             tags_range,
//         })
//     }
// }
//
// impl std::fmt::Display for Analysis {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.line)
//     }
// }
//
// impl serde::Serialize for Analysis {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_str(&self.line)
//     }
// }
//
// impl Analysis {
//     fn from_line(line: &str) -> Result<Self, ()> {
//         // uses the std::str::FromStr implementation above
//         line.parse()
//     }
//
//     pub fn to_json(&self) -> serde_json::Value {
//         serde_json::json!({
//             "lemma": self.lemma(),
//             "wordform": self.wordform(),
//             "pos": self.pos(),
//             "tags": self.tags_vec(),
//         })
//     }
//
//     pub fn wordform(&self) -> &str {
//         // SAFETY range is in bounds, and does not lie on a utf-8 boundary,
//         // because we made it in from_line() by using str.find() on specific
//         // characters. If the characters aren't found, the entire Analysis
//         // doesn't exist in the first place.
//         unsafe {
//             self.line
//                 .as_str()
//                 .get_unchecked(self.wordform_range.clone())
//         }
//         //let std::ops::Range { start, end } = self.wordform_range;
//         //unsafe {
//         //    let ptr = self.line.as_ptr().add(start);
//         //    let slice = core::slice::from_raw_parts(ptr, end - start);
//         //    core::str::from_utf8_unchecked(slice)
//         //}
//     }
//
//     pub fn lemma(&self) -> &str {
//         // SAFETY see wordform() safety note
//         unsafe { &self.line.as_str().get_unchecked(self.lemma_range.clone()) }
//     }
//
//     pub fn pos(&self) -> &str {
//         // SAFETY see wordform() safety note
//         unsafe { &self.line.as_str().get_unchecked(self.pos_range.clone()) }
//     }
//
//     pub fn tags_str(&self) -> &str {
//         // SAFETY see wordform() safety note
//         unsafe { &self.line.as_str().get_unchecked(self.tags_range.clone()) }
//     }
//
//     pub fn tags_vec(&self) -> Vec<&str> {
//         self.tags_str().split('+').collect()
//     }
//
//     /// Is the analysis a derivation
//     pub fn is_derivation(&self) -> bool {
//         self.tags_str().contains("Der")
//     }
//
//     /// Is the analysis a compound
//     pub fn is_compund(&self) -> bool {
//         self.tags_str().contains('#') && !self.tags_str().starts_with(&['+', '#'])
//     }
// }
//
// pub fn analyses_raw_to_vec(raw: &str) -> Vec<Analysis> {
//     raw.split('\n')
//         .filter_map(|line| line.parse::<crate::analysis::Analysis>().ok())
//         .collect::<Vec<_>>()
// }
//
// /// Try to analyze an `input` string with the analyzer for language `lang`.
// /// `tokenize` determines if the input is run through the tokenizer before
// /// analysis, or not.
// pub fn analyze<'a>(input: &str, lang: &str, tokenize: bool) -> Result<String, String> {
//     let analyzer_gt_desc_hfstol =
//         get_langfile(lang, "analyser-gt-desc.hfstol").ok_or_else(|| {
//             format!(
//                 "language not supported \
//             (analyser-gt-desc.hfstol doesn't exist for language {}",
//                 lang
//             )
//         })?;
//
//     let results = if tokenize {
//         // Asked for tokenized analysis, try to do it, but if we don't
//         // have the tokeniser file, fall back to non-tokenised analysis
//         match get_langfile(&lang, "tokeniser-disamb-gt-desc.pmhfst") {
//             Some(tokdisamb_file) => {
//                 run_fun!(
//                     echo "$input" |
//                     hfst-tokenize -q $tokdisamb_file |
//                     hfst-lookup -q --beam=0 $analyzer_gt_desc_hfstol
//                 )
//             }
//             None => {
//                 run_fun!(
//                     echo "$input" |
//                     hfst-lookup -q --beam=0 $analyzer_gt_desc_hfstol
//                 )
//             }
//         }
//     } else {
//         trace!("XXX");
//         let x = run_fun!(
//             echo "$input" |
//             hfst-lookup -q --beam=0 $analyzer_gt_desc_hfstol
//         );
//         trace!("YYY");
//         x
//     };
//
//     let analyses_string = results.map_err(|e| e.to_string())?;
//     Ok(analyses_string)
// }
//
// fn analyze2(input: &str, lang: &str) -> Result<Vec<(String, f32)>, ()> {
//     let t0 = Instant::now();
//     //let analyses = transducer.lookup(input);
//     let analyses = vec![];
//     let t = Instant::now().duration_since(t0);
//     trace!("analysis(using hfst) took: {t:?}");
//     Ok(analyses)
// }
//
// pub async fn analyze_async<'a>(input: &str, lang: &str, tokenize: bool) -> Result<String, String> {
//     let input = input.to_owned();
//     let lang = lang.to_owned();
//     let t0 = std::time::Instant::now();
//     let analyses = tokio::task::spawn_blocking(move || analyze(&input, &lang, tokenize))
//         .await
//         .unwrap()?;
//     trace!("analysis took: {}ms", t0.elapsed().as_millis());
//     Ok(analyses)
// }
