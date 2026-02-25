use std::io::Read;

/// Read out all paragraphs in a .docx Word document
pub fn read_docx_text(data: Vec<u8>) -> Option<String> {
    let Ok(docx) = docx_rs::read_docx(&data) else {
        return None;
    };

    let text = docx
        .document
        .children
        .iter()
        .filter_map(|doc_child| {
            if let docx_rs::DocumentChild::Paragraph(paragraph) = doc_child {
                Some(paragraph)
            } else {
                None
            }
        })
        .map(|paragraph| paragraph.raw_text())
        .collect::<Vec<_>>()
        .join("\n");

    let text = newline_converter::dos2unix(&text).to_string();

    Some(text)
}

/// decompress gzipped data
pub fn gunzip(data: Vec<u8>) -> std::io::Result<Vec<u8>> {
    let mut gz = flate2::bufread::GzDecoder::new(&data[..]);
    let mut buf = Vec::new();
    let _n = gz.read_to_end(&mut buf);
    Ok(buf)
}

// /// Used to check if a query param is present, and equal to some string
// /// that any user would say is true, like "1", "y", "yes", and "true",
// /// case-insensitively. The query param would not be considered trueish if
// /// explicitly set to something like "n", or "no", so we just test a few
// /// specific strings.
// pub fn query_param_is_trueish(
//     params: &std::collections::HashMap<String, String>,
//     key: &str,
// ) -> bool {
//     params
//         .get(key)
//         .filter(|v| crate::common_url::trueish_query_param(v))
//         .is_some()
// }

/// std::Iterator::intersperse() is nightly only, so lets just throw together
/// a quick way to join an iterator of strings with newlines, without having to
/// allocate a vec and .join()....
///
/// Example
///
/// ```
/// let v = vec!["these", "are", "some", "words"];
///
/// let s = String::from_iter(intersperse_newlines(v.iter()));
/// assert_eq!(s, String::from("these\nare\nsome\nwords"));
/// ```
pub fn intersperse_newlines<'a>(
    mut it: impl Iterator<Item = &'a str>,
) -> impl Iterator<Item = &'a str> {
    let mut did = false;
    let mut done = false;

    std::iter::from_fn(move || {
        if done {
            None
        } else if did {
            did = false;
            Some("\n")
        } else {
            match it.next() {
                Some(next) => {
                    did = true;
                    Some(next)
                }
                None => {
                    done = true;
                    None
                }
            }
        }
    })
}
