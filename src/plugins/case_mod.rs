#[cfg(feature = "case_mod")]
use convert_case::{Case, Casing};

#[cfg(feature = "case_mod")]
pub fn parse(i: &mut String, command: &str) {
    let tmp = match command {
        "snake-case" => i.to_case(Case::Snake),
        "PascalCase" => i.to_case(Case::Pascal),
        "camelCase" => i.to_case(Case::Camel),
        _ => i.to_string(),
    };
    i.clear();
    i.insert_str(0, &tmp);
}
