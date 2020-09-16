extern crate nom;

use crate::constants::{IN_ISOLATOR, OUT_ISOLATOR, SEPARATOR};

pub fn parse(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer_in = input;
    let mut buffer_out = String::new();

    while !buffer_in.is_empty() {
        let result = parse_block(&buffer_in).expect("Cannot parse block");
        buffer_out = format!("{}{}", buffer_out, result.1);
        buffer_in = result.0.to_string();
    }

    Ok(buffer_out)
}

// * Parse block
fn parse_block<'a>(i: &'a str) -> nom::IResult<&'a str, String> {
    let mut result = String::new();
    let (mut i, res) = not_special(i)?;
    result.insert_str(result.len(), res);
    if let Ok((input, res)) = special(i) {
        let (_, tag_replaced) = replace_tag(res)?;
        result.insert_str(result.len(), &tag_replaced);
        i = input;
    }

    Ok((i, result))
}

// Parse special block
// E.G `{{something}} here` => (`something`, ` here`)
fn special(i: &str) -> nom::IResult<&str, &str> {
    nom::sequence::delimited(
        nom::bytes::complete::tag(IN_ISOLATOR),
        is_not_isolator_and_trim,
        nom::bytes::complete::tag(OUT_ISOLATOR),
    )(i)
}

// Anything not IN_ISOLATOR
// E.G `something {{example}}` => (`example}}`, `something `)
fn not_special(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::is_not(IN_ISOLATOR)(i)
}

// Anything not OUT_ISOLATOR (and trim whitespaces)
// E.G ` example }} something` => (`example`, ` something`)
fn is_not_isolator_and_trim(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::is_not(OUT_ISOLATOR)(i).map(|(i, res)| (i, res.trim()))
}

// Parse => token and optional plugin command
// E.G `token` => (`token`, None)
// E.G `token | command` => (`token`, Some(`command`))
fn parse_token_command(i: &str) -> nom::IResult<&str, Option<&str>> {
    let (i, token) = nom::bytes::complete::is_not(SEPARATOR)(i)?;
    let command: nom::IResult<&str, &str, nom::error::VerboseError<&str>> =
        nom::bytes::complete::tag(SEPARATOR)(i);
    let command = command.ok().map(|(command, _)| command.trim());
    Ok((token.trim(), command))
}

// * Replace tag from config and apply plugin function if exist
fn replace_tag(i: &str) -> nom::IResult<&str, String> {
    let (token, command) = parse_token_command(i)?;

    let map: std::collections::HashMap<&str, &str> = [
        ("name", "Johan Planchon"),
        ("Licence", "MIT"),
        ("test", "middle"),
    ]
    .iter()
    .cloned()
    .collect();

    let value = map.get(token).unwrap_or_else(|| -> &&str {
        eprintln!("Key not found in config");
        &""
    });

    Ok(("", plugins(value, command.unwrap_or(""))))
}

// * Plugins
#[cfg(feature = "case_mod")]
use crate::plugins::case_mod;

#[allow(unused_variables)]
fn plugins(i: &str, command: &str) -> String {
    #[allow(unused_mut)]
    let mut result = String::from(i);
    #[cfg(feature = "case_mod")]
    case_mod::parse(&mut result, command);

    result
}
