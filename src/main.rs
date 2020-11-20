#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;

use anyhow::{Context, Result};
use human_panic::setup_panic;

/* === LOCAL IMPORTS === */
// utils
mod app;
mod errors;
// subcommands
mod generate;
mod install;
mod list;
mod new;
mod uninstall;
mod utils;
/* === LOCAL IMPORTS === */

pub const DEFAULT_ASK: bool = false;
// Install dir is in the $HOME user directory
pub const INSTALL_DIR: &str = ".boilrs";
pub const CONFIG_FILE_NAME: &str = "templates.toml";
pub const TEMPLATE_IGNORE_FILE: &str = ".ignore";
pub const TEMPLATE_DIR_NAME: &str = "template";
pub const TEMPLATE_CONFIG_NAME: &str = "project.toml";

type StandardResult<T> = std::result::Result<T, errors::BoilrError>;

/* SIZE OPTIMISATION */
#[cfg(feature = "smaller")]
use std::alloc::System;

#[cfg(feature = "smaller")]
#[global_allocator]
static A: System = System;
/* SIZE OPTIMISATION */

fn main() -> Result<()> {
    setup_panic!();

    // * Creating cli app
    let cli = app::init_app().get_matches();

    match cli.subcommand() {
        ("generate", Some(args)) => generate::generate(&args).context(format!(
            "Failed to generate the template using `args`: {:?}",
            args
        ))?,
        ("new", Some(args)) => new::new(&args).context(format!(
            "Failed to create new config using `args`: {:?}",
            args
        ))?,
        ("install", Some(args)) => install::install(args).context(format!(
            "Failed to install the template using `args`: {:?}",
            args
        ))?,
        ("uninstall", Some(args)) => uninstall::uninstall(args).context(format!(
            "Failed to uninstall the template using `args`: {:?}",
            args
        ))?,
        ("list", Some(args)) => list::list(args).context(format!(
            "Failed to list the templates using `args`: {:?}",
            args
        ))?,
        // ("download", Some(_args)) => unimplemented!(),
        // ("update", Some(_args)) => unimplemented!(),
        _ => (),
    }

    Ok(())
}
