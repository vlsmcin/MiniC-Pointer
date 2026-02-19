//! Literal parsers for MiniC.

use crate::ir::ast::Literal as AstLiteral;
use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag, take_while1},
    character::complete::{char, digit1},
    combinator::{map, opt, value},
    sequence::{preceded, tuple},
    IResult,
};

/// A parsed literal value.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

/// Parse an integer literal (optional minus, decimal digits).
/// Fails if followed by .digit (to reject "12.34" as integer).
pub fn integer_literal(input: &str) -> IResult<&str, i64> {
    let (rest, neg) = opt(char('-'))(input)?;
    let (rest, digits) = digit1(rest)?;
    // Reject "12.34" - integer must not be followed by .digit
    if rest.starts_with('.') {
        let mut chars = rest.chars();
        chars.next(); // skip '.'
        if chars.next().map_or(false, |c| c.is_ascii_digit()) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Digit,
            )));
        }
    }
    let value: i64 = digits.parse().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
    })?;
    Ok((rest, if neg.is_some() { -value } else { value }))
}

/// Parse a float literal.
pub fn float_literal(input: &str) -> IResult<&str, f64> {
    nom::number::complete::double(input)
}

/// Parse a string literal with escapes.
pub fn string_literal(input: &str) -> IResult<&str, String> {
    alt((
        // Empty string
        value(String::new(), tag("\"\"")),
        // Non-empty string with optional escapes
        map(
            tuple((
                preceded(
                    char('"'),
                    escaped_transform(
                        take_while1(|c: char| c != '"' && c != '\\'),
                        '\\',
                        alt((
                            value("\\", tag("\\")),
                            value("\"", tag("\"")),
                            value("\n", tag("n")),
                            value("\t", tag("t")),
                        )),
                    ),
                ),
                char('"'),
            )),
            |(s, _)| s,
        ),
    ))(input)
}

/// Parse a boolean literal.
pub fn boolean_literal(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}

/// Parse any literal (tries each type in order).
/// Order: boolean, integer, float, string (integer before float so "42" parses as Int).
pub fn literal(input: &str) -> IResult<&str, Literal> {
    alt((
        map(boolean_literal, Literal::Bool),
        map(integer_literal, Literal::Int),
        map(float_literal, Literal::Float),
        map(string_literal, Literal::Str),
    ))(input)
}

impl From<Literal> for AstLiteral {
    fn from(l: Literal) -> Self {
        match l {
            Literal::Int(n) => AstLiteral::Int(n),
            Literal::Float(f) => AstLiteral::Float(f),
            Literal::Str(s) => AstLiteral::Str(s),
            Literal::Bool(b) => AstLiteral::Bool(b),
        }
    }
}
