use crate::langmodel_files::get_langfile;
use crate::util::{gunzip, read_docx_text};
use axum::{
    extract::Json,
    response::{IntoResponse, Response},
};
use base64::{Engine as _, engine::general_purpose};
use cmd_lib::spawn_with_output;
use http::StatusCode;
use nix::{
    sys::signal::{Signal, kill},
    unistd::Pid,
};
use serde::Deserialize;
use std::io::Write;
use tempfile::NamedTempFile;
use tracing::info;

const UE: StatusCode = StatusCode::UNPROCESSABLE_ENTITY;
const ISE: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;

#[derive(Deserialize)]
pub struct InputBody {
    typ: String,
    lang: String,
    data: String,
}

pub async fn lemma_count_endpoint(
    Json(InputBody { typ, lang, data }): Json<InputBody>,
) -> Response {
    let text: String = match typ.as_str() {
        "text" => data,
        "text+gz+b64" => {
            let Ok(data) = general_purpose::STANDARD.decode(data) else {
                return (UE, "could not base64 decode data").into_response();
            };
            let Ok(data) = gunzip(data) else {
                return (UE, "failed to gunzip data").into_response();
            };
            let Ok(text) = String::from_utf8(data) else {
                return (UE, "text not valid utf-8").into_response();
            };
            text
        }
        "docx" => {
            let Ok(data) = general_purpose::STANDARD.decode(data) else {
                return (UE, "could not base64 decode data").into_response();
            };
            let Some(text) = read_docx_text(data) else {
                return (UE, "could not read docx file").into_response();
            };
            text
        }
        _ => return (UE, "'typ' field must be text, text+gz+b64 or docx").into_response(),
    };

    let Some(tokdisamb) = get_langfile(&lang, "tokeniser-disamb-gt-desc.pmhfst") else {
        return (
            UE,
            format!("no tokeniser-disamb-gt-desc.pmhfst for lang {lang}"),
        )
            .into_response();
    };
    let Some(disambcg) = get_langfile(&lang, "disambiguator.cg3")
        .or_else(|| get_langfile(&lang, "disambiguator.bin"))
    else {
        return (UE, format!("no disambiguator.(cg3|bin) for lang {lang}")).into_response();
    };

    let Ok(temp_file) = NamedTempFile::new() else {
        return (ISE, "can't create tempfile").into_response();
    };
    let Ok(_) = temp_file.as_file().write_all(text.as_bytes()) else {
        return (ISE, "can't write to tempfile").into_response();
    };
    let path = temp_file.path().to_path_buf();

    let cut_delim = "-d\"";
    let sed_trim_starting_whitespace = "s/^ *//";

    let (_drop_detector, drop_signal) = DropDetector::new();

    let mut children = match spawn_with_output!(
        cat $path |
        hfst-tokenise -cg $tokdisamb |
        vislcg3 -g $disambcg |
        grep -v "^[\"]" |
        cut $cut_delim -f2 |
        uniq |
        grep -v "^[:]" |
        sort |
        uniq -c |
        sed $sed_trim_starting_whitespace |
        grep "[a-zæøåA-ZÆØÅ]" |
        sort -k1,1nr -k2,2
    ) {
        Ok(children) => children,
        Err(e) => return (ISE, format!("failed to spawn pipeline: {e}")).into_response(),
    };
    let pids: Vec<i32> = children.pids().iter().map(|&pid| pid as i32).collect();
    let pipeline_handle =
        tokio::task::spawn_blocking(move || children.wait_with_output().map_err(|e| e.to_string()));

    let (tx, mut rx) = tokio::sync::oneshot::channel();
    let _x = tokio::spawn(async move {
        let _ = drop_signal.await;
        let done = rx.try_recv().unwrap_or(false);
        if !done {
            for pid in pids {
                if kill(Pid::from_raw(pid as i32), Signal::SIGKILL).is_err() {
                    info!("failed to kill (pid={pid})");
                }
            }
            info!("connection dropped");
        }
    });

    let result = match pipeline_handle.await {
        Ok(text) => (StatusCode::OK, text).into_response(),
        Err(err) => (ISE, err.to_string()).into_response(),
    };

    tx.send(true).unwrap();

    result
}

struct DropDetector(Option<tokio::sync::oneshot::Sender<()>>);

impl DropDetector {
    fn new() -> (Self, tokio::sync::oneshot::Receiver<()>) {
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        (Self(Some(tx)), rx)
    }
}

impl Drop for DropDetector {
    fn drop(&mut self) {
        if let Some(sender) = self.0.take() {
            let _ = sender.send(());
        }
    }
}
