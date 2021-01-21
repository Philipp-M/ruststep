use super::{literal::*, util::*};

mod operator;
mod qualifier;

use operator::*;
use qualifier::*;

/// Unary expresion, e.g. `x` or binary expression `x + y`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<Base, Op> {
    Unary(Base),
    Binary { op: Op, arg1: Base, arg2: Base },
}

impl<Base, Op> From<Base> for Expr<Base, Op> {
    fn from(base: Base) -> Self {
        Expr::Unary(base)
    }
}

pub type Expression = Expr<SimpleExpression, RelOpExtended>;
pub type SimpleExpression = Expr<Term, AddLikeOp>;
pub type Term = Expr<Factor, MultiplicationLikeOp>;
pub type Factor = Expr<SimpleFactor, PowerOp>;

fn expr<'a, Base, Op>(
    input: &'a str,
    base_parse: impl EsprParser<'a, Base>,
    op_parser: impl EsprParser<'a, Op>,
) -> ParseResult<'a, Expr<Base, Op>>
where
    Base: 'a,
    Op: 'a,
{
    tuple((
        base_parse.clone(),
        opt(tuple((spaces, op_parser, base_parse))),
    ))
    .map(|(base, opt)| {
        if let Some((_, op, arg2)) = opt {
            Expr::Binary {
                op,
                arg1: base,
                arg2,
            }
        } else {
            Expr::Unary(base)
        }
    })
    .parse(input)
}

/// 216 expression = simple_expression \[ rel_op_extended simple_expression \] .
pub fn expression(input: &str) -> ParseResult<Expression> {
    expr(input, simple_expression, rel_op_extended)
}

/// 305 simple_expression = term { add_like_op term } .
pub fn simple_expression(input: &str) -> ParseResult<SimpleExpression> {
    expr(input, term, add_like_op)
}

/// 325 term = factor { multiplication_like_op factor } .
pub fn term(input: &str) -> ParseResult<Term> {
    expr(input, factor, multiplication_like_op)
}

/// 217 factor = simple_factor \[ `**` simple_factor \] .
pub fn factor(input: &str) -> ParseResult<Factor> {
    expr(input, simple_factor, power_op)
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimaryOrExpression {
    Primary(Primary),
    Expression(Box<Expression>), // to avoid recusive definition
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleFactor {
    PrimaryOrExpression {
        unary_op: Option<UnaryOp>,
        primary_or_expression: PrimaryOrExpression,
    },
}

impl From<Primary> for SimpleFactor {
    fn from(primary: Primary) -> Self {
        SimpleFactor::PrimaryOrExpression {
            unary_op: None,
            primary_or_expression: PrimaryOrExpression::Primary(primary),
        }
    }
}

impl From<Expression> for SimpleFactor {
    fn from(expression: Expression) -> Self {
        SimpleFactor::PrimaryOrExpression {
            unary_op: None,
            primary_or_expression: PrimaryOrExpression::Expression(Box::new(expression)),
        }
    }
}

/// 306 simple_factor = aggregate_initializer
///                   | entity_constructor
///                   | enumeration_reference
///                   | interval
///                   | query_expression
///                   | ( \[ unary_op \] ( `(` expression `)` | primary ) ) .
pub fn simple_factor(input: &str) -> ParseResult<SimpleFactor> {
    // FIXME Add aggregate_initializer
    // FIXME Add entity_constructor
    // FIXME Add enumeration_reference
    // FIXME Add interval
    // FIXME Add query_expression
    tuple((
        opt(tuple((unary_op, spaces)).map(|(op, _)| op)),
        alt((
            primary.map(|primary| PrimaryOrExpression::Primary(primary)),
            tuple((char('('), expression, char(')'))).map(|(_open, expression, _close)| {
                PrimaryOrExpression::Expression(Box::new(expression))
            }),
        )),
    ))
    .map(
        |(unary_op, primary_or_expression)| SimpleFactor::PrimaryOrExpression {
            unary_op,
            primary_or_expression,
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Finish;

    #[test]
    fn expr() {
        let (residual, e) = super::expression("1 - 2").finish().unwrap();
        dbg!(e);
        assert_eq!(residual, "");
    }

    #[test]
    fn simple_factor() {
        let (residual, (p, _remarks)) = super::simple_factor("123").finish().unwrap();
        assert_eq!(p, Primary::Literal(Literal::Real(123.0)).into());
        assert_eq!(residual, "");

        let (residual, (p, _remarks)) = super::simple_factor("-123").finish().unwrap();
        assert_eq!(
            p,
            SimpleFactor::PrimaryOrExpression {
                unary_op: Some(UnaryOp::Minus),
                primary_or_expression: PrimaryOrExpression::Primary(Primary::Literal(
                    Literal::Real(123.0)
                )),
            }
        );
        assert_eq!(residual, "");
    }

    #[test]
    fn simple_factor_expression() {
        let (residual, (expr, _remarks)) = super::expression("1 + 2").finish().unwrap();
        assert_eq!(residual, "");

        let (residual, (sf, _remarks)) = super::simple_factor("(1 + 2)").finish().unwrap();
        assert_eq!(residual, "");
        match sf {
            SimpleFactor::PrimaryOrExpression {
                unary_op: _,
                primary_or_expression,
            } => match primary_or_expression {
                PrimaryOrExpression::Expression(e) => assert_eq!(*e, expr),
                _ => panic!("Must be expression"),
            },
        }
    }

    #[test]
    fn primary() {
        let (residual, (p, _remarks)) = super::primary("123").finish().unwrap();
        assert_eq!(p, Literal::Real(123.0).into());
        assert_eq!(residual, "");
    }
}
