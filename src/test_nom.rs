use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_till, take_until, take_while_m_n};
use nom::character::complete::{char, digit0, digit1};
use nom::combinator::{map, map_res, opt, rest};
use nom::error::context;
use nom::multi::{many0, separated_list0};
use nom::number::complete::double;
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::{AsChar, IResult};
use nom::Parser;

fn num_list(i: &str) -> IResult<&str, Vec<f64>> {
    delimited(char('('), separated_list0(char(','), double), char(')')).parse(i)
}


#[test]
fn run(){
    let mut parser = delimited(tag::<&str, &str, nom::error::Error<&str>>("("), tag("abc"), tag(")"));
/// assert_eq!(parser.parse("(abc)"), Ok(("", "abc")));
/// assert_eq!(parser.parse("(abc)def"), Ok(("def", "abc")));
/// assert_eq!(parser.parse(""), Err(Err::Error(("", ErrorKind::Tag))));
/// assert_eq!(parser.parse("123"), Err(Err::Error(("123", ErrorKind::Tag))));

    println!("{:?}",num_list("(1125,223)"));
}