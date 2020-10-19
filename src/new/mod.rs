use crate::app::{notify, overwrite_if_exist};
use crate::{app, StandardResult, TEMPLATE_CONFIG_NAME, TEMPLATE_DIR_NAME, TEMPLATE_IGNORE_FILE};
use clap::ArgMatches;
use std::env::current_dir;
use std::fs::{create_dir, File};
use std::path::PathBuf;

pub fn new(args: &ArgMatches) -> StandardResult<()> {
	let output_path = PathBuf::from(Option::unwrap_or(
			args.value_of("output"),
			current_dir()
			.expect("Cannot access current directory")
			.to_str()
			.expect("Cannot convert current directory path to string"),
			));
	let template_name = args.value_of("name").expect("Name cli arg not found");

	if !output_path.is_dir() {
		app::error("Output path is not a directory!");
		std::process::exit(-1);
	}

	let full_output_path = output_path.join(template_name);

	overwrite_if_exist(&full_output_path)?;

	create_dir(&full_output_path)?;
	create_dir(&full_output_path.join(TEMPLATE_DIR_NAME))?;
	File::create(&full_output_path.join(TEMPLATE_IGNORE_FILE))?;
	File::create(&full_output_path.join(TEMPLATE_CONFIG_NAME))?;

	notify(&format!(
			"New blank template created at \"{}\"",
			full_output_path
			.to_str()
			.expect("Cannot convert path to str")
			));

	Ok(())
}
