use std::fs::remove_dir_all;

use clap::ArgMatches;

use crate::app::{alert, notify};
use crate::errors::BoilrError;
use crate::utils::config::ConfigIO;
use crate::StandardResult;
use std::path::PathBuf;

pub fn uninstall(args: &ArgMatches) -> StandardResult<()> {
    let template_name = args.value_of("name").expect("Name cli arg not found");

    let mut io = ConfigIO::new()?;

    let template_index = io.find_index(|t| t.name == template_name);

    if let Some(template_index) = template_index {
        let template_path = PathBuf::from(io.config.templates.remove(template_index).path);
        remove_dir_all(&template_path).map_err(|source| BoilrError::WriteError {
            source,
            path: template_path,
        })?;
        io.write_config()?;

        notify(&["Template '", template_name, "' successfully uninstalled"].concat());
    } else {
        alert("Cannot uninstall, config not found!");
    }

    Ok(())
}
