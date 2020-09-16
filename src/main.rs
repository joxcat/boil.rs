#![allow(dead_code)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

use human_panic::setup_panic;
use std::path::PathBuf;

/* === LOCAL IMPORTS === */
mod app;
mod config;
mod constants;
mod errors;
mod parser;
mod plugins;
/* === LOCAL IMPORTS === */
pub const INPUT: &str = "before {{test}} next {{ test }} or {{ test | PascalCase }}";
pub type StandardResult<T> = Result<T, errors::Error>;

fn main() -> StandardResult<()> {
    setup_panic!();

    let cli = app::init_app().get_matches();
    let template_path = cli
        .value_of("template")
        .expect("Template cli arg not found");
    let _output_path = cli.value_of("output").expect("Output path not found");

    let config = config::parse_config(PathBuf::from(template_path).join("project.toml"))?;
    println!("{:?}", config);
    Ok(())
}
