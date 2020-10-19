use crate::{StandardResult, INSTALL_DIR};
use clap::{App, AppSettings, Arg, SubCommand};
use console::style;
use dialoguer::Confirm;
use dirs::home_dir;
use std::fs::{create_dir, remove_dir_all, remove_file, File};
use std::path::PathBuf;

const GLOBAL_SETTINGS: &[AppSettings] = &[
	AppSettings::ArgRequiredElseHelp,
	AppSettings::ColoredHelp,
	AppSettings::ColorAuto,
];

pub fn init_app<'a, 'b>() -> App<'a, 'b> {
	App::new("boilrs")
		.about(crate_description!())
		.author(crate_authors!())
		.bin_name("boilrs")
		.version(crate_version!())
		.subcommand(
			SubCommand::with_name("new")
				.visible_aliases(&["n", "blank", "create"])
				.settings(GLOBAL_SETTINGS)
				.about("Create a blank template")
				.arg(
					Arg::with_name("output")
						.long("output")
						.short("o")
						.value_name("OUTPUT_DIRECTORY")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("name")
						.long("name")
						.short("n")
						.value_name("NAME")
						.required(true)
						.takes_value(true)
						.help("Name of the folder where the template will be stored"),
				),
		)
		.subcommand(
			SubCommand::with_name("install")
				.visible_aliases(&["i", "add"])
				.settings(GLOBAL_SETTINGS)
				.about("Install a boilrs template in cache")
				.arg(
					Arg::with_name("name")
						.long("name")
						.short("n")
						.value_name("NAME")
						.required(true)
						.takes_value(true)
						.help("Name of the template in cache"),
				)
				.arg(
					Arg::with_name("path")
						.long("template")
						.short("t")
						.value_name("TEMPLATE_PATH")
						.takes_value(true)
						.help("Path to the template to install (directory) default to current dir"),
				),
		)
		.subcommand(
			SubCommand::with_name("update")
				.visible_alias("up")
				.settings(GLOBAL_SETTINGS)
				.about("Update the cache of a boilrs template"),
		)
		.subcommand(
			SubCommand::with_name("download")
				.visible_aliases(&["d", "dl"])
				.settings(GLOBAL_SETTINGS)
				.about("Download a template from github / gitlab / direct download link"),
		)
		.subcommand(
			SubCommand::with_name("list")
				.visible_aliases(&["l", "ls"])
				.settings(GLOBAL_SETTINGS)
				.about("List cached templates"),
		)
		.subcommand(
			SubCommand::with_name("uninstall")
				.visible_aliases(&["u", "rm", "remove"])
				.settings(GLOBAL_SETTINGS)
				.about("Uninstall cached template"),
		)
		.subcommand(
			SubCommand::with_name("generate")
				.visible_aliases(&["g", "gen"])
				.settings(GLOBAL_SETTINGS)
				.about("Generate project from template")
				.arg(
					Arg::with_name("template")
						.long("template")
						.short("t")
						.value_name("TEMPLATE_NAME")
						.required(true)
						.takes_value(true)
						.display_order(3)
						.help("Name of the template in cache"),
				)
				.arg(
					Arg::with_name("output")
						.long("output")
						.short("o")
						.value_name("OUTPUT_DIRECTORY")
						.takes_value(true)
						.display_order(2),
				)
				.arg(
					Arg::with_name("name")
						.long("name")
						.short("n")
						.value_name("NAME")
						.required(true)
						.takes_value(true)
						.display_order(1)
						.help("Name of the folder generated"),
				),
		)
		.settings(GLOBAL_SETTINGS)
}

pub fn notify(info: &str) {
	println!("{} {}", style("[✓]").bold().green(), style(info).bold())
}

pub fn error(info: &str) {
	eprintln!("{} {}", style("[✗]").bold().red(), style(info).bold())
}

pub fn alert(info: &str) {
	println!("{} {}", style("[!]").bold().yellow(), style(info).bold())
}

pub fn ask(question: &str) -> StandardResult<bool> {
	Ok(Confirm::new()
		.default(crate::DEFAULT_ASK)
		.with_prompt(format!(
			"{} {}",
			style("[?]").bold().cyan(),
			style(question).bold()
		))
		.interact()?)
}

pub fn overwrite_if_exist(path: &PathBuf, die_if_dont: bool) -> StandardResult<()> {
	let path_str = path.to_str().expect("Cannot display output path");

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
			std::process::exit(-2);
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
		std::process::exit(-3);
	}

	Ok(())
}
