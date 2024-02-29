use colored::*;

use crate::wgsl_error::WgslError;
use std::path::Path;
use crate::naga::Naga;
use walkdir::WalkDir;

pub struct OutputMessage {
    pub is_err: bool,
    pub text: String,
}

impl OutputMessage {
    pub fn success(path: &Path) -> Self {
        let succes = "Success".bright_green().bold();
        OutputMessage {
            is_err: false,
            text: format!("✅ {} {}", succes, path.display()),
        }
    }

    pub fn error(path: &Path, error: WgslError) -> Self {
        let err_text = match error {
            WgslError::ParserErr { error, line, pos } => {
                let arrow = "-->".blue();
                let location = format!("{}:{}:{}", path.display(), line, pos);
                let error = format!("{}: {}", "error".red().bold(), error);

                format!("{} {}\n{}", arrow, location, error)
            }
            WgslError::ValidationErr { error, emitted, .. } => {
                format!("❌ {} \n{:?} {}", path.display(), error, emitted)
            }
            err => {
                format!("❌ {} \n{:#?}", path.display(), err)
            }
        };

        Self {
            is_err: true,
            text: err_text,
        }
    }
}

pub fn run() -> i32 {
    let root_dir = std::fs::canonicalize("./").unwrap();

    let mut validator = Naga::new();

    let dir_walk = WalkDir::new(&root_dir);
    let dir_walk = dir_walk.into_iter().filter_entry(|e| {
        let path = e.path();

        if !path.is_dir() {
            path.extension().map(|ext| ext == "wgsl").unwrap_or(false)
        } else {
            true
        }
    });

    let mut messages = Vec::new();

    for entry in dir_walk {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if !path.is_dir() {
                    let msg = match validator.validate_wgsl(path) {
                        Ok(_) => {
                            let path = path.strip_prefix(&root_dir).unwrap_or(path);
                            OutputMessage::success(path)
                        }
                        Err(err) => {
                            let path = path.strip_prefix(&root_dir).unwrap_or(path);
                            OutputMessage::error(path, err)
                        }
                    };

                    messages.push(msg);
                }
            }
            Err(err) => {
                messages.push(OutputMessage {
                    is_err: true,
                    text: format!("{:?}", err),
                });
            }
        }
    }

    messages.sort_by(|a, b| {
        if a.is_err && b.is_err {
            std::cmp::Ordering::Equal
        } else if a.is_err {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });

    let mut exit_code = 0;

    for msg in messages {
        println!("{}", msg.text);
        if msg.is_err {
            exit_code = 1;
        }
    }

    exit_code
}
