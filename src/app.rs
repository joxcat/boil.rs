use crate::StandardResult;
use clap::{App, AppSettings, Arg, SubCommand};
use console::style;
use dialoguer::Confirm;

const GLOBAL_SETTINGS: &[AppSettings] = &[
    AppSettings::ArgRequiredElseHelp,
    AppSettings::ColoredHelp,
    AppSettings::ColorAuto,
];

pub fn init_app<'a, 'b>() -> App<'a, 'b> {
    App::new("boilrs")
        .about(crate_description!())
        .author(crate_authors!())
        .bin_name("boilrs")
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("new")
                .visible_aliases(&["n", "blank", "create"])
                .settings(GLOBAL_SETTINGS),
        )
        .subcommand(
            SubCommand::with_name("install")
                .visible_aliases(&["i", "add"])
                .settings(GLOBAL_SETTINGS),
        )
        .subcommand(
            SubCommand::with_name("download")
                .visible_aliases(&["d", "dl"])
                .settings(GLOBAL_SETTINGS),
        )
        .subcommand(
            SubCommand::with_name("list")
                .visible_aliases(&["l", "ls"])
                .settings(GLOBAL_SETTINGS),
        )
        .subcommand(
            SubCommand::with_name("uninstall")
                .visible_aliases(&["u", "rm", "remove"])
                .settings(GLOBAL_SETTINGS),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .visible_aliases(&["g", "gen"])
                .settings(GLOBAL_SETTINGS)
                .arg(
                    Arg::with_name("template")
                        .long("template")
                        .short("t")
                        .value_name("TEMPLATE_NAME")
                        .required(true)
                        .takes_value(true)
                        .display_order(3),
                )
                .arg(
                    Arg::with_name("output")
                        .long("output")
                        .short("o")
                        .value_name("OUTPUT_DIRECTORY")
                        .takes_value(true)
                        .display_order(2),
                )
                .arg(
                    Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .value_name("NAME")
                        .required(true)
                        .takes_value(true)
                        .display_order(1),
                ),
        )
        .settings(GLOBAL_SETTINGS)
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
