use super::{basis::*, combinator::*};
use crate::ast::*;

/// 251 literal = binary_literal | [logical_literal] | [real_literal] | [string_literal] .
///
/// Integer value, e.g. `23`, will be recognized as a real number `23.0`.
/// Use [integer_literal] if you wish to parse it as an integer.
pub fn literal(input: &str) -> ParseResult<Literal> {
    // FIXME binary_literal,
    alt((
        logical_literal.map(Literal::Logial),
        real_literal.map(Literal::Real),
        string_literal.map(Literal::String),
    ))
    .parse(input)
}

/// 255 logical_literal = `FALSE` | `TRUE` | `UNKNOWN` .
pub fn logical_literal(input: &str) -> ParseResult<Logical> {
    alt((
        value(Logical::True, tag("TRUE")),
        value(Logical::False, tag("FALSE")),
        value(Logical::Unknown, tag("UNKNOWN")),
    ))
    .parse(input)
}

/// 141 integer_literal = digits .
///
/// Negative integer, e.g. `-23`,
/// will be represented by the combination of `-` unary operator and integer literal `23`
pub fn integer_literal(input: &str) -> ParseResult<u64> {
    remarked(nom::character::complete::digit1)
        .map(|d: &str| d.parse().unwrap())
        .parse(input)
}

/// 142 real_literal = integer_literal | ( digits `.` \[ digits \] \[ `e` \[ sign \] digits \] ) .
pub fn real_literal(input: &str) -> ParseResult<f64> {
    remarked(nom::number::complete::double).parse(input)
}

/// 310 string_literal = simple_string_literal | encoded_string_literal .
pub fn string_literal(input: &str) -> ParseResult<String> {
    alt((
        remarked(simple_string_literal),
        remarked(encoded_string_literal),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn integer_literal() {
        let (residual, (value, _remarks)) = super::integer_literal("123").finish().unwrap();
        assert_eq!(value, 123);
        assert_eq!(residual, "");
    }

    #[test]
    fn real_literal() {
        let (residual, (value, _remarks)) = super::real_literal("123").finish().unwrap();
        assert_eq!(value, 123.0);
        assert_eq!(residual, "");

        let (residual, (value, _remarks)) = super::real_literal("123.456").finish().unwrap();
        assert_eq!(value, 123.456);
        assert_eq!(residual, "");

        let (residual, (value, _remarks)) = super::real_literal("1.23e-5").finish().unwrap();
        assert_eq!(value, 1.23e-5);
        assert_eq!(residual, "");
    }
}
