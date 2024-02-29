use colored::*;

use cargo_wgsl::{naga::Naga, errors::WgslError};
use std::{io, path::Path};
use walkdir::WalkDir;

pub fn main() -> io::Result<()> {
    let root_dir = std::fs::canonicalize("./")?;

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

    for entry in dir_walk {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if !path.is_dir() {
                    match validator.validate_wgsl(path) {
                        Ok(_) => {
                            let path = path.strip_prefix(&root_dir).unwrap_or(path);
                            success(path)
                        }
                        Err(err) => {
                            let path = path.strip_prefix(&root_dir).unwrap_or(path);
                            error(path, err)
                        }
                    }
                }
            }
            Err(err) => eprintln!("{:?}", err),
        }
    }

    Ok(())
}

pub fn success(path: &Path) {
    let succes = "Success".bright_green().bold();
    eprintln!("✅ {} {}", succes, path.display())
}

pub fn error(path: &Path, error: WgslError) { 
    match error {
        WgslError::Parse { error, line, pos } => {
            eprintln!(
                "{} {}:{line}:{pos}\n{}: {}",
                "=>".blue(),
                path.display(),
                "error".red().bold(),
                error
            )
        }
        WgslError::Validate { error, emitted, .. } => eprintln!("❌ {} \n{:?} {}", path.display(), error, emitted),
        err => eprintln!("❌ {} \n{:#?}", path.display(), err),
    }
}
