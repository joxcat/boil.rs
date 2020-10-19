use super::parser::FileContent;
use crate::app::overwrite_if_exist;
use crate::{StandardResult, TEMPLATE_DIR_NAME};
use indicatif::ProgressBar;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::PathBuf;
use walkdir::DirEntry;

pub fn reconstruct(
	from_path: &PathBuf,
	path: &PathBuf,
	folders: &[DirEntry],
) -> StandardResult<()> {
	overwrite_if_exist(&path)?;
	create_dir(path)?;
	let progress = ProgressBar::new_spinner();
	progress.set_message("[3/4] Reconstructing template directories...");

	for folder in progress.wrap_iter(folders.iter()) {
		let new_path = path.join(
			folder
				.path()
				.strip_prefix(from_path.join(TEMPLATE_DIR_NAME))?,
		);
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
