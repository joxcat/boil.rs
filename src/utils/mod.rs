use crate::{StandardResult, INSTALL_DIR};
use crate::errors::BoilrError;
use crate::app::{error, alert};
use std::path::PathBuf;
use std::env::current_dir;
use std::fs::{create_dir, File, remove_dir_all, remove_file};
use dirs::home_dir;
use clap::ArgMatches;

pub fn overwrite_if_exist(path: &PathBuf, die_if_dont: bool) -> StandardResult<()> {
	let path_str = path.to_str().ok_or(BoilrError::StrError)?;

	if path.exists() {
		if crate::app::ask(&format!(
			"File/Directory already exist at \"{}\" do you want to overwrite it?",
			path_str
		))? {
			if path.is_dir() {
				alert(&format!("Overwriting dir recursively at \"{}\"", path_str));
				remove_dir_all(&path)?;
			} else {
				alert(&format!("Overwriting file at \"{}\"", path_str));
				remove_file(&path)?;
			}
		} else if die_if_dont {
			error("Please change output path if you do not want to overwrite it!");
			std::process::exit(-1);
		}
	}
	Ok(())
}

pub fn check_if_install_dir_exist() -> StandardResult<()> {
	let path = home_dir()
		.expect("Cannot find home directory")
		.join(INSTALL_DIR);

	if !path.exists() {
		create_dir(&path)?;
		create_dir(&path.join("templates"))?;
		File::create(&path.join("templates.toml"))?;
	} else if !path.is_dir() {
		error(&format!(
			"Install dir (\"{}\") is not a directory",
			INSTALL_DIR
		));
		return Err(BoilrError::NotADirectory { path }.into());
	}

	Ok(())
}

pub fn to_output_path(args: &ArgMatches) -> StandardResult<PathBuf> {
	Ok(PathBuf::from(Option::unwrap_or(
		args.value_of("output"),
		current_dir()
			.map_err(|_| BoilrError::AccessCurrentDirError)?
			.to_str()
			.ok_or(BoilrError::StrError)?,
	)))
}