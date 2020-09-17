use crate::parser::TeraFilter;
use std::collections::HashMap;
use tera::{Result, Value};

pub fn all<'a>() -> Vec<(&'static str, TeraFilter<'a>)> {
    let mut result = Vec::new();
    result.push(("snake_case", &case::snake_case as TeraFilter<'a>));
    result.push(("PascalCase", &case::pascal_case as TeraFilter<'a>));
    result.push(("camelCase", &case::camel_case as TeraFilter<'a>));
    result
}

pub mod case {
    use super::{HashMap, Result, Value};
    use convert_case::{Case, Casing};

    pub fn snake_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("snake-case", "value", String, value);
        Ok(Value::String(s.to_case(Case::Snake)))
    }

    pub fn pascal_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("PascalCase", "value", String, value);
        Ok(Value::String(s.to_case(Case::Pascal)))
    }
    pub fn camel_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
        let s = tera::try_get_value!("camelCase", "value", String, value);
        Ok(Value::String(s.to_case(Case::Camel)))
    }
}
