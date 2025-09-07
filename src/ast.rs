use eros::{Context, IntoDynTracedError, Result};
use std::str::FromStr;

use crate::token::Token;
use chumsky::{
    input::{Stream, ValueInput},
    prelude::*,
};
use logos::Logos;

use crate::value::Value;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Call(String, Vec<Ast>),
    Constant(Value),
}

pub fn parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Ast, extra::Err<Rich<'tokens, Token>>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    recursive(|ast| {
        let constant = select! {
            Token::Int(n) => Ast::Constant(Value::Int(BigInt::from_str(&n).unwrap())),
            Token::Float(n) => Ast::Constant(Value::Float(n.parse().unwrap())),
            Token::String(s) => Ast::Constant(Value::String(s)),
        };

        let term = ast
            .clone()
            .delimited_by(just(Token::LeftParen), just(Token::RightParen));

        let func_call = select! { Token::Var(name) => name }
            .then(
                ast.clone()
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .collect()
                    .delimited_by(just(Token::LeftParen), just(Token::RightParen)),
            )
            .map(|(name, args)| Ast::Call(name, args));

        let primary = choice((constant.clone(), term.clone(), func_call.clone())).clone();

        let mul_div_expr = primary.clone().foldl(
            choice((
                just(Token::Mul).to("mul".to_string()),
                just(Token::Div).to("div".to_string()),
            ))
            .then(primary.clone())
            .repeated(),
            |lhs, (op, rhs)| Ast::Call(op, vec![lhs, rhs]),
        );

        let add_sub_expr = mul_div_expr.clone().foldl(
            choice((
                just(Token::Add).to("add".to_string()),
                just(Token::Sub).to("sub".to_string()),
            ))
            .then(mul_div_expr.clone())
            .repeated(),
            |lhs, (op, rhs)| Ast::Call(op, vec![lhs, rhs]),
        );

        choice((add_sub_expr, func_call))
    })
}

pub fn parse_input(input: &str) -> Result<Ast> {
    let tokens = Token::lexer(&input).spanned().map(|(tok, span)| match tok {
        Ok(t) => (t, span.into()),
        Err(_) => (Token::Error, span.into()),
    });

    let token_stream =
        Stream::from_iter(tokens).map((0..input.len()).into(), |(tok, span): (_, _)| (tok, span));

    parser()
        .parse(token_stream)
        .into_result()
        .map_err(|e| ParseError::from_vec_rich(e))
        .traced_dyn()
        .context("ParseError")
}

#[derive(Debug, Clone)]
struct ParseError(String);

impl ParseError {
    fn from_vec_rich(errs: Vec<Rich<'_, Token>>) -> Self {
        let mut msg = String::new();
        for err in errs {
            msg.push_str(&format!("{}\n", err));
        }
        ParseError(msg)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

#[test]
fn test_parse() {
    use ariadne::{Color, Label, Report, ReportKind, Source};
    use chumsky::input::Stream;
    use logos::Logos;

    let input = r#"func(1) + foo(1*3, "23") / (8 + 9)+2"#;
    let tokens = Token::lexer(&input)
        .spanned()
        .map(|(tok, span)| match tok {
            Ok(t) => (t, span.into()),
            Err(_) => (Token::Error, span.into()),
        })
        .inspect(|(t, _)| println!("{}", t));

    let token_stream =
        Stream::from_iter(tokens).map((0..input.len()).into(), |(tok, span): (_, _)| (tok, span));

    match parser().parse(token_stream).into_result() {
        Ok(expr) => {
            dbg!(&expr);
        }

        Err(errs) => {
            for err in errs {
                Report::build(ReportKind::Error, ((), err.span().into_range()))
                    .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                    .with_code(3)
                    .with_message(err.to_string())
                    .with_label(
                        Label::new(((), err.span().into_range()))
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .eprint(Source::from(&input))
                    .unwrap();
            }
        }
    }
}
