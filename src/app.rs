use crate::StandardResult;
use clap::{App, AppSettings, Arg};
use console::style;
use glob::Pattern;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

/*
1. Need create blank
2. Need download from github / git+ssh
3. Need list downloaded
*/
pub fn init_app<'a, 'b>() -> App<'a, 'b> {
    App::new("boilrs")
        .about(crate_description!())
        .author(crate_authors!())
        .bin_name("boilrs")
        .version(crate_version!())
        .usage("boilrs -t <TEMPLATE_PATH> -o <OUTPUT_PATH>")
        .arg(
            Arg::with_name("template")
                .short("t")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .required(true)
                .takes_value(true),
        )
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::ColoredHelp,
            AppSettings::ColorAuto,
        ])
}

pub fn scan_dir(template_dir: &PathBuf) -> StandardResult<(Vec<DirEntry>, Vec<DirEntry>)> {
    let mut folders = Vec::new();
    let mut files = Vec::new();
    let rules = generate_ignore_rules(template_dir);

    for entry in WalkDir::new(template_dir.join("template"))
        .follow_links(true)
        .into_iter()
        .filter_entry(|d| filters(d, &rules, template_dir))
    {
        match entry {
            Ok(e) => {
                if e.path().is_file() {
                    files.push(e);
                } else if e.path() != template_dir.join("template") {
                    folders.push(e);
                }
            }
            Err(e) => return Err(e.into()),
        };
    }

    Ok((folders, files))
}

fn filters(entry: &DirEntry, ignore_rules: &[Pattern], base_path: &PathBuf) -> bool {
    for rule in ignore_rules {
        if let Ok(stripped_path) = entry.path().strip_prefix(base_path.join("template")) {
            if rule.matches_path(stripped_path) {
                return false;
            }
        }
    }
    true
}

fn generate_ignore_rules(template_dir: &PathBuf) -> Vec<Pattern> {
    let mut ignore_rules = Vec::new();
    if let Ok(f) = std::fs::read_to_string(template_dir.join(".ignore")) {
        for line in f.lines() {
            match Pattern::new(line) {
                Ok(p) => ignore_rules.push(p),
                Err(_) => alert(&format!(
                    "\"{}\" in .ignore is not a valid unix pattern",
                    line
                )),
            };
        }
    }

    ignore_rules
}

pub fn notify(info: &str) {
    println!("{} {}", style("[✓]").bold().green(), style(info).bold())
}

pub fn alert(info: &str) {
    eprintln!("{} {}", style("[✗]").bold().red(), style(info).bold())
}
