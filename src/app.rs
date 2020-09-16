use clap::{App, AppSettings, Arg};
use console::style;

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

pub fn notify(info: &str) {
    println!("{} {}", style("[✓]").bold().green(), style(info).bold())
}

pub fn alert(info: &str) {
    eprintln!("{} {}", style("[✗]").bold().red(), style(info).bold())
}
