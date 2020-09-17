use std::collections::HashMap;
use tera::{Context, Result, Tera, Value};

pub type TeraFilter<'a> =
    &'a (dyn (Fn(&Value, &HashMap<String, Value>) -> Result<Value>) + Send + Sync);

pub fn parse(text: &str, config: &HashMap<String, String>) -> crate::StandardResult<String> {
    let mut tera = Tera::default();
    let mut context = Context::new();
    for (key, value) in config {
        context.insert(key, &value);
    }

    for (filter_name, filter) in filters_plugins() {
        tera.register_filter(filter_name, filter);
    }

    Ok(tera.render_str(text, &context)?)
}

// * Plugins
#[cfg(feature = "case_mod")]
use crate::plugins::case_mod;

#[allow(unused_variables)]
fn filters_plugins<'a>() -> Vec<(&'static str, TeraFilter<'a>)> {
    let mut result = Vec::new();
    #[cfg(feature = "case_mod")]
    result.extend(case_mod::all());
    result
}
