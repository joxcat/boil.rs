use crate::app::notify;
use crate::{app, StandardResult, TEMPLATE_CONFIG_NAME, TEMPLATE_DIR_NAME, TEMPLATE_IGNORE_FILE};
use crate::errors::BoilrError;
use crate::utils::{overwrite_if_exist, to_output_path};
use clap::ArgMatches;
use std::fs::{create_dir, File};

pub fn new(args: &ArgMatches) -> StandardResult<()> {
	let output_path = to_output_path(args)?;

	let template_name = args.value_of("name").ok_or(BoilrError::ArgNotFound)?;

	if !output_path.is_dir() {
		app::error("Output path is not a directory!");
		return Err(BoilrError::NotADirectory { path: output_path }.into());
	}

	let full_output_path = output_path.join(template_name);

	overwrite_if_exist(&full_output_path, true)?;

	create_dir(&full_output_path)?;
	create_dir(&full_output_path.join(TEMPLATE_DIR_NAME))?;
	File::create(&full_output_path.join(TEMPLATE_IGNORE_FILE))?;
	File::create(&full_output_path.join(TEMPLATE_CONFIG_NAME))?;

	notify(&format!(
		"New blank template created at \"{}\"",
		full_output_path
			.to_str()
			.ok_or(BoilrError::StrError)?
	));

	Ok(())
}
