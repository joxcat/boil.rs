use crate::StandardResult;
use clap::{App, AppSettings, Arg};
use console::style;
use dialoguer::Confirm;
use glob::Pattern;
use indicatif::ProgressBar;
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
        .usage("boilrs --name <NAME> --template <TEMPLATE_PATH> --output <OUTPUT_PATH>")
        .arg(
            Arg::with_name("template")
                .long("template")
                .short("t")
                .required(true)
                .takes_value(true)
                .display_order(3),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .takes_value(true)
                .display_order(2),
        )
        .arg(
            Arg::with_name("name")
                .long("name")
                .short("n")
                .required(true)
                .takes_value(true)
                .display_order(1),
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

    let walkdir_iter = WalkDir::new(template_dir.join("template"))
        .follow_links(true)
        .into_iter()
        .filter_entry(|d| filters(d, &rules, template_dir));

    let progress = ProgressBar::new_spinner();
    progress.set_message("[1/4] Scanning files and folders in template...");

    for entry in progress.wrap_iter(walkdir_iter) {
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

    progress.finish_and_clear();
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

pub fn error(info: &str) {
    eprintln!("{} {}", style("[✗]").bold().red(), style(info).bold())
}

pub fn alert(info: &str) {
    println!("{} {}", style("[!]").bold().yellow(), style(info).bold())
}

pub fn ask(question: &str) -> StandardResult<bool> {
    Ok(Confirm::new()
        .default(crate::DEFAULT_ASK)
        .with_prompt(format!(
            "{} {}",
            style("[?]").bold().blue(),
            style(question).bold()
        ))
        .interact()?)
}
