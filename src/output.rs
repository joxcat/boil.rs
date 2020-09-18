use crate::app::{alert, error};
use crate::parser::FileContent;
use crate::StandardResult;
use indicatif::ProgressBar;
use std::fs::{create_dir, remove_dir_all, remove_file, File};
use std::io::Write;
use std::path::PathBuf;
use walkdir::DirEntry;

pub fn reconstruct(
    from_path: &PathBuf,
    path: &PathBuf,
    folders: &[DirEntry],
) -> StandardResult<()> {
    let full_path_str = path.to_str().expect("Cannot display output path");

    if path.exists() {
        if crate::app::ask(&format!(
            "File/Directory already exist at \"{}\" do you want to overwrite it?",
            full_path_str
        ))? {
            if path.is_dir() {
                alert(&format!(
                    "Overwriting dir recursively at \"{}\"",
                    full_path_str
                ));
                remove_dir_all(&path)?;
            } else {
                alert(&format!("Overwriting file at \"{}\"", full_path_str));
                remove_file(&path)?;
            }
        } else {
            error("Please change output path if you do not want to overwrite it!");
            std::process::exit(2);
        }
    }
    create_dir(path)?;
    let progress = ProgressBar::new_spinner();
    progress.set_message("[3/4] Reconstructing template directories...");

    for folder in progress.wrap_iter(folders.iter()) {
        let new_path = path.join(folder.path().strip_prefix(from_path.join("template"))?);
        create_dir(new_path)?;
    }

    progress.finish_and_clear();
    Ok(())
}

pub fn write(path: &PathBuf, files: &[(PathBuf, FileContent)]) -> StandardResult<()> {
    let progress = ProgressBar::new_spinner();
    progress.set_message("[4/4] Writing files to output...");

    for (file_path, file_content) in progress.wrap_iter(files.iter()) {
        match file_content {
            FileContent::Text(content) => {
                File::create(path.join(file_path))?.write_all(content.as_bytes())?
            }
            FileContent::Binary(content) => {
                File::create(path.join(file_path))?.write_all(content)?
            }
        }
    }

    progress.finish_and_clear();
    Ok(())
}
