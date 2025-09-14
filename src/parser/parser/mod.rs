use std::str::FromStr;

use chumsky::input::ValueInput;
use chumsky::prelude::*;
use num_bigint::BigInt;

use super::lexer::Token;
use crate::ast::{Ast, Value};

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
