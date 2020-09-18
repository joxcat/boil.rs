use crate::StandardResult;
use console::style;
use dialoguer::{Confirm, Input, Select};
use std::collections::HashMap;
use std::path::PathBuf;
use tera::Value;

pub fn parse_config(path: PathBuf) -> StandardResult<HashMap<String, Value>> {
    let toml: toml::Value = toml::from_str(&std::fs::read_to_string(path)?)?;
    let mut config: HashMap<String, Value> = HashMap::new();

    for (key, val) in toml.as_table().expect("Cannot parse config file") {
        match val {
            toml::Value::Array(vals) => {
                config.insert(key.clone(), Value::String(ask_within_array(key, vals)?))
            }
            toml::Value::String(el) => config.insert(
                key.clone(),
                Value::String(ask_with_default_string(key, el)?),
            ),
            toml::Value::Boolean(b) => {
                config.insert(key.clone(), Value::Bool(ask_confirmation(key, b)?))
            }
            _ => {
                crate::app::alert(&format!(
                    "Unsupported variable type in the configuration: \"{}\" with value \"{}\"",
                    key,
                    val.to_string()
                ));
                None
            }
        };
    }

    Ok(config)
}

fn ask_with_default_string(key: &str, default: &str) -> StandardResult<String> {
    Ok(Input::<String>::new()
        .default(default.to_string())
        .show_default(false)
        .with_prompt(format!(
            "{} {} \"{}\" {}",
            style("[?]").bold().blue(),
            style("Please choose a value for").bold(),
            style(key).bold(),
            style(format!("[default: \"{}\"]", default)).blue()
        ))
        .interact()?)
}

fn ask_within_array(key: &str, arr: &[toml::Value]) -> StandardResult<String> {
    let arr = arr
        .iter()
        .map(|el| {
            el.as_str()
                .expect("Internal error while prompting for array element")
        })
        .collect::<Vec<&str>>();
    Ok(arr
        .get(
            Select::new()
                .default(0)
                .with_prompt(format!(
                    "{} {} \"{}\" {}",
                    style("[?]").bold().blue(),
                    style("Please choose an option for").bold(),
                    style(key).bold(),
                    style(format!(
                        "[default: \"{}\"]",
                        arr.first().expect("Array in config is empty")
                    ))
                    .blue()
                ))
                .items(&arr)
                .interact()?,
        )
        .expect("Internal error while prompting for array element")
        .to_string())
}

fn ask_confirmation(key: &str, default: &bool) -> StandardResult<bool> {
    Ok(Confirm::new()
        .default(*default)
        .show_default(false)
        .with_prompt(format!(
            "{} {} \"{}\" {}",
            style("[?]").bold().blue(),
            style("Please choose (true/false) for").bold(),
            style(key).bold(),
            style(format!("[default: \"{}\"]", default)).blue()
        ))
        .interact()?)
}
