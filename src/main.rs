#![allow(dead_code)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

use human_panic::setup_panic;
use std::env::current_dir;
use std::path::PathBuf;

/* === LOCAL IMPORTS === */
mod app;
mod config;
mod errors;
mod output;
mod parser;
mod plugins;
/* === LOCAL IMPORTS === */

pub const DEFAULT_ASK: bool = false;

pub type StandardResult<T> = Result<T, errors::Error>;

fn main() -> StandardResult<()> {
    setup_panic!();

    // * Creating cli app
    let cli = app::init_app().get_matches();

    // * Parsing cli args
    let template_path = PathBuf::from(
        cli.value_of("template")
            .expect("Template cli arg not found"),
    );
    let template_path_str = template_path
        .to_str()
        .expect("Cannot convert template path to str");

    let output_path = PathBuf::from(Option::unwrap_or(
        cli.value_of("output"),
        current_dir()
            .expect("Cannot access current directory")
            .to_str()
            .expect("Cannot convert current directory path to string"),
    ));
    if !output_path.is_dir() {
        app::error("Output path is not a directory!");
        std::process::exit(1);
    }

    let name = cli.value_of("name").expect("Name arg not found");

    // * Generate placeholders from project.toml
    let config = config::parse_config(template_path.join("project.toml"))?;

    // * Scanning files and folders in template dir
    let (folder_entries, file_entries) = app::scan_dir(&template_path)?;
    app::notify(&format!(
        "Scanned files and folders in \"{}\"",
        template_path_str
    ));

    // * Parse files using Tera
    let process_files = parser::process_files(&template_path, file_entries, &config)?;
    app::notify(&format!("Parsed files in \"{}\"", template_path_str));

    // * Reconstruct template in output
    let full_output_path = output_path.join(name);
    let full_output_path_str = full_output_path
        .to_str()
        .expect("Cannot convert output_path to str");

    output::reconstruct(&template_path, &full_output_path, &folder_entries)?;
    app::notify(&format!(
        "Reconstructed template directories at \"{}\"",
        full_output_path_str
    ));

    output::write(&full_output_path, &process_files)?;

    app::notify(&format!(
        "Successfully created template at \"{}\"",
        full_output_path_str
    ));

    Ok(())
}
