#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

use human_panic::setup_panic;

/* === LOCAL IMPORTS === */
// utils
mod app;
mod errors;
// subcommands
mod generate;
mod install;
mod new;
/* === LOCAL IMPORTS === */

pub const DEFAULT_ASK: bool = false;
pub const INSTALL_DIR: &str = "$HOME/.boilrs";

pub type StandardResult<T> = Result<T, errors::Error>;

fn main() -> StandardResult<()> {
    setup_panic!();

    // * Creating cli app
    let cli = app::init_app().get_matches();

    match cli.subcommand() {
        ("generate", Some(args)) => generate::generate(&args)?,
        ("new", Some(_args)) => unimplemented!(),
        ("install", Some(_args)) => unimplemented!(),
        ("uninstall", Some(_args)) => unimplemented!(),
        ("list", Some(_args)) => unimplemented!(),
        ("download", Some(_args)) => unimplemented!(),
        _ => (),
    }

    Ok(())
}
