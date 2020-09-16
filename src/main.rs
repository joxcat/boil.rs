#![allow(dead_code)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

use human_panic::setup_panic;

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

    let _cli = app::init_app().get_matches();

    Ok(())
}
