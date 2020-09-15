extern crate nom;

const INPUT: &str = "before {{test}} next";

fn special(i: &str) -> nom::IResult<&str, &str> {
    nom::sequence::delimited(
        nom::bytes::complete::tag("{{"),
        nom::bytes::complete::is_not("}}"),
        nom::bytes::complete::tag("}}"),
    )(i)
}

fn not_special(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::is_not("{{")(i)
}

fn parser(i: &str) -> nom::IResult<&str, String> {
    let mut result = String::new();
    let (i, res) = not_special(i)?;
    result = format!("{}{}", result, res);
    let (i, res) = special(i)?;
    result = format!(
        "{}{}",
        result,
        match res {
            "test" => "middle",
            _ => "",
        }
    );
    Ok((i, result))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer_in = INPUT;
    let mut buffer_out = String::new();
    while buffer_in.len() != 0 {
        let result = parser(buffer_in)?;
        println!("{:?}", result);
        buffer_in = result.0;
        buffer_out = format!("{}{}", buffer_out, result.1);
    }
    println!("{}", INPUT);
    println!("{}", buffer_out);
    Ok(())
}
