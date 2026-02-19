//! Identifier parser for MiniC.

use nom::{
    bytes::complete::{take_while, take_while1},
    combinator::{recognize, verify},
    sequence::pair,
    IResult,
};

/// Parse an identifier (variable name).
/// Must start with letter or underscore; subsequent chars may be letter, digit, or underscore.
/// Rejects reserved words `true` and `false`.
pub fn identifier(input: &str) -> IResult<&str, &str> {
    let id_parser = recognize(pair(
        take_while1(|c: char| c.is_alphabetic() || c == '_'),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ));
    verify(id_parser, |s: &str| s != "true" && s != "false")(input)
}
