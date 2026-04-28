use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use axum::extract::Query;
use axum::response::{IntoResponse, Json, Response};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, span, Level};

use crate::common_url::{BoolQueryParam, Format, FormatQueryParam};

/// Total number of languages we have. Change this if we add or remove any.
const NUM_LANGS: usize = 51;

pub const LANGS: [&str; NUM_LANGS] = [
    "bxr", "chr", "ciw", "cor", "crk", "deu", "est", "evn", "fao", "fin", "fit", "fkv", "gle",
    "hdn", "hun", "ipk", "izh", "kal", "kca", "koi", "kom", "kpv", "lav", "liv", "lut", "mdf",
    "mhr", "mns", "mrj", "myv", "nio", "nno", "nob", "olo", "rmf", "rup", "rus", "sje", "sma",
    "sme", "smj", "smn", "sms", "som", "swe", "tkl", "udm", "vep", "vot", "vro", "yrk",
];

// A static hashmap to find where a file for a given language is located
// (lang, filename) -> file path
pub static LANGFILES: Lazy<RwLock<HashMap<(String, String), PathBuf>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

// The parsed langmodel_files.yaml, with definitions of all language model
// files, where they are located in the repositories, and which pipelines
// they are used for
pub static LANGMODEL_FILES: Lazy<LangmodelFiles> = Lazy::new(|| {
    let yaml = include_str!("langmodel_files.yaml");
    serde_yaml::from_str(yaml).unwrap()
});

#[derive(Serialize, Deserialize, Debug)]
pub struct LangmodelFiles {
    pub files: Vec<LangmodelFile>,
}

impl LangmodelFiles {
    /*
    fn all_filenames(&self) -> impl Iterator<Item = &str> {
        self.files
            .iter()
            .flat_map(|langmodel_file| {
                let alt = langmodel_file.alt_filename.map(|a| a.as_str());
                [Some(langmodel_file.filename.as_str()), alt]
            })
            .filter_map(|opt| opt)
    }
    */

    /// Is a given filename a language model file?
    fn is_langmodel_file(&self, filename: &str, lang: &str) -> bool {
        self.files
            .iter()
            .flat_map(|langmodel_file| {
                let replaced = langmodel_file.filename.replace("{lang}", lang);
                // TODO this technically assumes alt_filename never contains
                // "{lang}" - which for now is true, but may not always be
                [Some(replaced), langmodel_file.alt_filename.clone()]
            })
            .flatten()
            .find(|langmodel_filename| langmodel_filename == filename)
            .is_some()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LangmodelFile {
    /// The name of this file. May include the special "{lang}" substring,
    /// which would be replaced by the language it is in. For example, there
    /// is a file "paradigm_full.sme.txt", in the "sme" repository. In the
    /// yaml definition, it would be listed as "paradigm_full.{lang}.txt",
    /// so that the "paradigm_full.smj.txt" file in the "smj" repo could be
    /// found in the same way.
    pub filename: String,

    /// Sometimes, an alternative file can be used instead, interchangably.
    /// Typically a source file can be used instead of a binary compiled file.
    /// Example: .bin is fine if .cg3 isn't found (or vice versa).
    alt_filename: Option<String>,

    /// Path to where this file is located in the git repository.
    repo_path: String,
    alt_repo_path: Option<String>,

    /// List of pipeline methods this file is used for. For example "generate",
    /// or "paradigm"
    methods: Vec<String>,

    /// Not really used here, but declared for other tools. This is an Option
    /// here because the field may not be present in the yaml. None means the
    /// same as Some(false) - i.e. "does not require build".
    requires_build: Option<bool>,

    /// If `requires_build`, which build flags do we need to pass to
    /// ./configure. Can be empty, indicating no build args are needed.
    build_flags: Option<Vec<String>>,

    /// If the file is located in a shared repository, it is shared among a
    /// given list of languages. This property lists those languages.
    shared_by: Option<Vec<String>>,

    /// If `shared_by`, this field specifies the repository name where it
    /// resides. None if it isn't a shared file.
    repo: Option<String>,
}

impl LangmodelFile {
    /// Try to find this file on the current system, from the `base` path.
    /// Returns a 2-tuple of (path, filename), the path the file was found at,
    /// and the filename it was found under, which just so happens to be
    /// the exact same type as the key of the LANGFILES hashmap.
    #[tracing::instrument(level = "trace", skip(self, base), fields(file = self.filename))]
    pub fn find_on_system(&self, base: &Path, repo: &str) -> Option<(PathBuf, String)> {
        // try WP_LANGFOLDER/REPO/FILENAME
        let filename = self.filename.replace("{lang}", repo);
        let path = base.join(repo).join(&filename);
        if path.is_file() {
            //debug!(path = ?path, "found file");
            return Some((path, filename));
        }

        // try WP_LANGFOLDER/REPO/bin/FILENAME (how it is on gtweb)
        let path = base.join(repo).join("bin").join(&filename);
        if path.is_file() {
            //debug!(path = ?path, "found file");
            return Some((path, filename));
        }

        // try WP_LANGFOLDER/REPO/REPO_PATH
        let repo_path = self.repo_path.replace("{lang}", repo);
        let path = base.join(repo).join(repo_path);
        if path.is_file() {
            let filename = path
                .file_name()
                .expect("this path has a filename")
                .to_str()
                .expect("this filename is a valid str")
                .to_string();
            //debug!(path = ?path, "found file");
            return Some((path, filename));
        }

        // if we have an alternate file..
        if let Some(ref alt_filename) = self.alt_filename {
            //try WP_LANGFOLDER/REPO/ALT_FILENAME
            let alt_filename = alt_filename.replace("{lang}", repo);
            let path = base.join(repo).join(&alt_filename);
            if path.is_file() {
                //debug!(path = ?path, "found file");
                return Some((path, alt_filename));
            }

            //try WP_LANGFOLDER/REPO/ALT_REPO_PATH
            let alt_repo_path = self.alt_repo_path.as_ref().unwrap().replace("{lang}", repo);
            let path = base.join(repo).join(&alt_repo_path);
            if path.is_file() {
                let filename = path
                    .file_name()
                    .expect("this path has a filename")
                    .to_str()
                    .expect("this filename is a valid str")
                    .to_string();
                //debug!(path = ?path, "found file");
                return Some((path, filename));
            }
        }

        //debug!("file not found");
        None
    }
}

/// Read the WP_LANGFOLDER, and fill the LANGFILES hashmap with the paths to
/// where the specified files in LANGMODEL_FILES are found.
#[tracing::instrument(level = "trace")]
pub fn load_langfiles() {
    let langfolder = PathBuf::from(&*WP_LANGFOLDER);
    let mut langfiles = LANGFILES.write().unwrap();

    LANGS.iter().for_each(|lang| {
        LANGMODEL_FILES.files.iter().for_each(|file| {
            if file.shared_by.is_some() {
                let repo = file.repo.as_ref().unwrap().as_str();
                if let Some((path, filename)) = file.find_on_system(&langfolder, repo) {
                    // the lookup key is always going to be the
                    // lang param of the search, so we need to
                    // insert this entry into all of the langs
                    // it's shared for
                    for lang in file.shared_by.as_ref().unwrap().iter() {
                        let key = (lang.to_string(), filename.clone());
                        let value = path.clone();
                        langfiles.insert(key, value);
                    }
                }
            }

            if let Some((path, filename)) = file.find_on_system(&langfolder, lang) {
                let key = (lang.to_string(), filename);
                langfiles.insert(key, path);
            };
        })
    });

    // Additional, special files, used by the /lemmalist, and
    // /unknown-lemmas-in-dict endpoints, customly hard coded here (for now)
    let lemmalist_pairs = [
        ("nob", "fin"),
        ("nob", "fkv"),
        ("nob", "sme"),
        ("nob", "sma"),
        ("fin", "nob"),
        ("fin", "sme"),
        ("fin", "smn"),
        ("fin", "sms"),
        ("fkv", "nob"),
        ("sma", "nob"),
        ("sme", "nob"),
        ("sme", "fin"),
        ("sme", "smn"),
        ("smn", "fin"),
        ("smn", "sme"),
    ];

    for (l1, l2) in lemmalist_pairs.iter() {
        let fst = format!("{}{}-all.hfst", l1, l2);
        let path_server = std::path::PathBuf::from(format!(
            "{}/{}/bin/{}{}-all.hfst",
            *WP_LANGFOLDER, l1, l1, l2
        ));
        //debug!(?path_server, "looking for file");
        let path = if path_server.exists() {
            //debug!(?path_server, "found file on server");
            path_server
        } else {
            std::path::PathBuf::from(format!("/usr/share/giella/{l1}/{l1}{l2}-all.hfst"))
        };

        langfiles.insert((l1.to_string(), fst), path);
    }
}

/// Try to add a new file to the LANGFILES hashmap.
/// This function is called by the file watcher when a new file is discovered.
pub fn add_langfile(path: &std::path::Path) {
    let span = span!(Level::DEBUG, "filewatcher: new file", path = ?path);
    let _guard = span.enter();

    let inner_path = path.strip_prefix(&*WP_LANGFOLDER).expect(
        "watched path is always in WP_LANGFOLDER, because it's the only path we ever watch!",
    );

    let mut path_components = inner_path.components();
    let Some(lang) = path_components.next() else {
        error!("new file was root folder - should never happen");
        return;
    };

    let Ok(lang) = path_component_as_normal_str(lang) else {
        return;
    };

    let Some(lang) = LANGS.contains(&lang).then_some(lang) else {
        // file was added inside a language we're not interested in, ignore
        info!("not interested (file or folder not in watched languages)");
        return;
    };

    let filename = path_components.next();
    match filename {
        Some(filename) => {
            let Ok(filename) = path_component_as_normal_str(filename) else {
                return;
            };
            if !LANGMODEL_FILES.is_langmodel_file(filename, lang) {
                info!("not interested (not a langmodel file)");
                return;
            };
            let mut langfiles = LANGFILES.write().unwrap();
            let key = (lang.to_string(), filename.to_owned());
            langfiles.insert(key, path.to_path_buf());
            info!("file added to LANGFILES");
        }
        None => {
            info!("new root language folder");
        }
    }
}

/// Remove a file from the LANGFILES hashmap.
/// This function is called by the file watcher when a file is deleted.
pub fn remove_langfile(path: &std::path::Path) {
    let span = span!(Level::DEBUG, "filewatcher: file deleted", path = ?path);
    let _guard = span.enter();

    let inner_path = path.strip_prefix(&*WP_LANGFOLDER).expect(
        "watched path is always in WP_LANGFOLDER, because it's the only path we ever watch!",
    );

    let mut path_components = inner_path.components();
    let Some(lang) = path_components.next() else {
        error!("new file was root folder - should never happen");
        return;
    };

    let Ok(lang) = path_component_as_normal_str(lang) else {
        return;
    };

    let Some(lang) = LANGS.contains(&lang).then_some(lang) else {
        // file was added inside a language we're not interested in, ignore
        info!("not interested (file or folder not in watched languages)");
        return;
    };

    let filename = path_components.next();
    match filename {
        Some(filename) => {
            let Ok(filename) = path_component_as_normal_str(filename) else {
                return;
            };
            if !LANGMODEL_FILES.is_langmodel_file(filename, lang) {
                info!("not interested (not a langmodel file)");
                return;
            };
            let mut langfiles = LANGFILES.write().unwrap();
            let key = (lang.to_string(), filename.to_owned());
            langfiles.remove(&key);
            info!("file removed from LANGFILES");
        }
        None => {
            let _langfiles = LANGFILES.write().unwrap();
            // remove all keys (lang, _)  - but to do this, we must know
            // the full key, meaning all files
            /*
            LANGMODEL_FILES.files
                .iter()
                .flat_
            let key =
            info!("entire language folder deleted");
            */
        }
    }
}

/// Try to get an owned PathBuf from a given (lang, file) pair, or None
/// if that file is missing for that language.
pub fn get_langfile(lang: &str, file: &str) -> Option<PathBuf> {
    LANGFILES
        .read()
        .unwrap()
        .get(&(lang.to_string(), file.to_string()))
        .cloned()
}

// The environment variable WP_LANGFOLDER
pub static WP_LANGFOLDER: Lazy<String> = Lazy::new(|| {
    dotenv::var("WP_LANGFOLDER").unwrap_or_else(|_| {
        eprintln!("environment variable WP_LANGFOLDER not set (or not unicode)");
        std::process::exit(2);
    })
});

#[derive(serde::Deserialize)]
pub struct Detailed {
    #[serde(
        default,
        deserialize_with = "crate::common_url::deserialize_boolqueryparam"
    )]
    detailed: Option<BoolQueryParam>,
}

/// Return information about which capabilities the API has. Mainly: which endpoints
/// are supported for which languages.
///
/// # Example:
/// ```json
/// {
///     "analyze": ["nob", "sme", "sma"],
///     "dependency": ["nob"],
/// }
/// ```
#[axum::debug_handler]
pub async fn info_endpoint(
    Query(format): Query<FormatQueryParam>,
    wants_detailed: Query<Detailed>,
) -> Response {
    let format = format.unwrap_or(Format::Json);
    let wants_detailed = wants_detailed
        .detailed
        .as_ref()
        .map(|detailed| detailed.0)
        .unwrap_or(false);

    let mut methods: HashMap<String, HashSet<String>> = HashMap::new();

    for file in LANGMODEL_FILES.files.iter() {
        for method in file.methods.iter() {
            let set = methods.entry(method.clone()).or_default();
            set.insert(file.filename.to_string());
        }
    }

    // method => HashMap<lang, Vec<(filename, bool)>>
    let mut detailed: HashMap<&str, HashMap<&str, Vec<(String, bool)>>> = HashMap::new();
    let mut simple: HashMap<&str, HashSet<String>> = HashMap::new();

    let langfiles = LANGFILES.read().unwrap();
    for lang in LANGS.iter() {
        for (method, files) in methods.iter() {
            let have_all_files = files.iter().all(|f| {
                let file = f.replace("{lang}", lang);
                langfiles.get(&(lang.to_string(), file)).is_some()
            });

            if have_all_files {
                let set = simple.entry(lang).or_default();
                set.insert(method.clone());
            }

            for file in files.iter() {
                let file = file.replace("{lang}", lang);
                let m = detailed.entry(method).or_default();
                let has_file = langfiles.get(&(lang.to_string(), file.clone())).is_some();
                let v = m.entry(lang).or_default();
                v.push((file, has_file));
            }
        }
    }

    match (format, wants_detailed) {
        (Format::Text, _) => {
            // TODO actually give text, and respect detailed
            Json(json!(detailed)).into_response()
        }
        (Format::Json, false) => Json(json!(simple)).into_response(),
        (Format::Json, true) => Json(json!(detailed)).into_response(),
        (Format::PrettyJson, false) => PrettyJson(simple).into_response(),
        (Format::PrettyJson, true) => PrettyJson(detailed).into_response(),
    }
}

struct PrettyJson<T>(T);

impl<T> IntoResponse for PrettyJson<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match serde_json::to_string_pretty(&self.0) {
            Ok(string) => string.into_response(),
            Err(_) => "error when serializing response".into_response(),
        }
    }
}

fn path_component_as_normal_str(component: std::path::Component<'_>) -> Result<&str, ()> {
    let std::path::Component::Normal(osstr) = component else {
        error!("always expect path component to be normal component");
        return Err(());
    };
    osstr.to_str().ok_or(())
}
