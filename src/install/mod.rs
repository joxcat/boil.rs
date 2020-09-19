use crate::app::check_if_install_dir_exist;
use crate::{StandardResult, INSTALL_DIR};
use clap::ArgMatches;
use dirs::home_dir;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::read_to_string;
use std::path::PathBuf;
use toml::Value;

pub fn install(args: &ArgMatches) -> StandardResult<()> {
    check_if_install_dir_exist()?;

    let template_path = PathBuf::from(Option::unwrap_or(
        args.value_of("path"),
        current_dir()
            .expect("Cannot access current directory")
            .to_str()
            .expect("Cannot convert current directory path to string"),
    ));

    let template_name = args.value_of("name").expect("Name cli arg not found");

    get_templates(
        &PathBuf::from(home_dir().expect("Cannot find HOME directory"))
            .join(INSTALL_DIR)
            .join("templates.toml"),
    )?;

    Ok(())
}

pub fn get_templates(path: &PathBuf) -> StandardResult<HashMap<String, String>> {
    let templates = HashMap::new();

    if let Some(Value::Array(templates_raw)) =
        toml::from_str::<Value>(&read_to_string(path)?)?.get("template")
    {
        println!("{:?}", templates_raw);
    }

    Ok(templates)
}
