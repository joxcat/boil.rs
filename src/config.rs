use std::collections::HashMap;

fn parse_config(path: &str) -> crate::StandardResult<HashMap<String, String>> {
    let config: HashMap<String, String> =
        toml::from_str(&std::fs::read_to_string(path)?).expect("Cannot parse config file");
    Ok(config)
}
