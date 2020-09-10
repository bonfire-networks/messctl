use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{anychar, space0, none_of},
    combinator::recognize,
    error::VerboseError,
    sequence::preceded,
    multi::{many1_count, many_till},
};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::types::*;

fn alnum_(c: char) -> bool {
    (c == '_') || c.is_ascii_lowercase() || c.is_ascii_digit()
}

fn parse_package(input: &str) -> IResult<&str, Package> {
    let (input, package) = take_while(alnum_)(input)?;
    let (input, _) = preceded(space0, preceded(tag("="), space0))(input)?;
    let (input, value) = preceded(tag("\""), recognize(many1_count(none_of("\""))))(input)?;
    let (post, _) = tag("\"")(input)?;
    Ok((post, Package::new(package, value)))
}

fn parse_enabled(input: &str) -> IResult<&str, Line> {
    let (input, pre) = space0::<&str, VerboseError<&str>>(input).unwrap();
    let (post, package) = parse_package(input)?;
    Ok(("", Line::Enabled(Enabled::new(pre, post, package))))
}

fn parse_disabled(input: &str) -> IResult<&str, Line> {
    let (post, (pre, package)) = many_till(anychar, parse_package)(input)?;
    Ok(("", Line::Disabled(Disabled::new(pre.iter().collect(), post, package))))
}

fn parse_ignored(input: &str) -> IResult<&str, Line> {
    Ok(("", Line::Ignored(input.to_owned())))
}

fn parse_line(input: &str) -> Line {
    alt((parse_enabled, parse_disabled, parse_ignored))(input).unwrap().1
}

pub fn parse_file(path: &Path) -> Vec<Line> {
    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents.lines().map(parse_line).collect()
    } else { Vec::new() }
}
