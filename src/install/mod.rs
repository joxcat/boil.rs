use crate::app::{alert, error};
use crate::errors::BoilrError;
use crate::utils::types::{Config, Template};
use crate::utils::{check_if_install_dir_exist, prompt_overwrite_if_exist, recursive_copy};
use crate::{StandardResult, INSTALL_DIR, TEMPLATE_CONFIG_NAME};
use clap::ArgMatches;
use dirs::home_dir;
use std::env::current_dir;
use std::fs::{read_to_string, write};
use std::path::PathBuf;

pub fn install(args: &ArgMatches) -> StandardResult<()> {
    check_if_install_dir_exist()?;
    let install_directory_path = &home_dir()
        .expect("Cannot find HOME directory")
        .join(INSTALL_DIR);

    let mut template_path = PathBuf::from(Option::unwrap_or(
        args.value_of("path"),
        current_dir()
            .expect("Cannot access current directory")
            .to_str()
            .ok_or(BoilrError::StrError)?,
    ));

    let template_name = args.value_of("name").expect("Name cli arg not found");

    if !template_path.join(TEMPLATE_CONFIG_NAME).is_file() {
        template_path = template_path.join(template_name);
    }

    if !template_path.join(TEMPLATE_CONFIG_NAME).is_file() {
        error(
            &[
                "Cannot find any valid template at ",
                template_path.to_str().ok_or(BoilrError::StrError)?,
            ]
            .concat(),
        );
        return Err(BoilrError::RuntimeError);
    }

    let mut config = get_config(&install_directory_path.join("templates.toml"))?;
    let templates = &config.templates;

    if let Some(template) = templates.iter().find(|t| t.name == template_name) {
        alert("This template is already installed");
        prompt_overwrite_if_exist(&install_directory_path.join(&template.path), false)?;
        config.templates.retain(|t| t.name != template_name);
    }

    // Recursive copy directory
    let target_path = install_directory_path.join("templates").join(template_name);

    recursive_copy(&template_path, &target_path).map_err(|source| BoilrError::CopyError {
        source: Box::new(source),
        from_path: template_path.clone(),
        to_path: target_path.clone(),
    })?;

    // Write templates.toml

    config.templates.push(Template {
        name: template_name.to_owned(),
        path: target_path.to_str().ok_or(BoilrError::StrError)?.to_owned(),
    });

    write(
        &install_directory_path.join("templates.toml"),
        toml::to_string(&config)
            .map_err(|source| BoilrError::TomlSerializeError {
                source,
                path: install_directory_path.join("templates.toml"),
            })?
            .as_bytes(),
    )
    .map_err(|source| BoilrError::WriteError {
        source,
        path: install_directory_path.join("templates.toml"),
    })?;

    Ok(())
}

pub fn get_config(path: &PathBuf) -> StandardResult<Config> {
    let config = toml::from_str::<Config>(&read_to_string(path).map_err(|source| {
        BoilrError::ReadError {
            source,
            path: path.clone(),
        }
    })?)
    .map_err(|source| BoilrError::TomlDeserializeError {
        source,
        path: path.clone(),
    })?;
    Ok(config)
}
