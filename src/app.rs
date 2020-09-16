use clap::{App, AppSettings, Arg};

pub fn init_app<'a, 'b>() -> App<'a, 'b> {
    App::new("boilrs")
        .about(crate_description!())
        .author(crate_authors!())
        .bin_name("boilrs")
        .version(crate_version!())
        .usage("boilrs -t <TEMPLATE_PATH> -o <OUTPUT_PATH>")
        .arg(Arg::with_name("template"))
        .arg(Arg::with_name("output"))
        .subcommand(
            App::new("new_blank")
                .about("Create a new blank template")
                .usage("boilrs new_blank <NAME>")
                .settings(&[AppSettings::ArgRequiredElseHelp, AppSettings::ColoredHelp]),
        )
        .settings(&[AppSettings::ArgRequiredElseHelp, AppSettings::ColoredHelp])
}
