use crate::utils::config::ConfigIO;
use crate::{StandardResult, INSTALL_DIR};
use clap::ArgMatches;

pub fn list(_args: &ArgMatches) -> StandardResult<()> {
    let io = ConfigIO::new()?;

    for template in io.iter() {
        println!("{} {}/{}", template.name, INSTALL_DIR, template.path);
    }

    Ok(())
}
