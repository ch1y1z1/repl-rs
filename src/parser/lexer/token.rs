use super::escape_character::ParseEscapeCharacter;
use logos::Logos;

#[derive(Logos, Clone, PartialEq, Debug)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(error = String)]
pub enum Token {
    Error,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token(",")]
    Comma,
    #[regex(r#"-?(?:0|[1-9]\d*)"#, |r| r.slice().to_owned())]
    Int(String),
    #[regex(r#"-?(?:(?:0|[1-9]\d*)\.\d*(?:[eE][+\-]?\d+)?|\.\d+(?:[eE][+\-]?\d+)?|(?:0|[1-9]\d*)(?:[eE][+\-]?\d+))"#,
        |r| r.slice().to_owned())]
    Float(String),
    #[regex(r#""(?:[^"\\\u0000-\u001F]|\\["\\/bfnrt]|\\u[0-9a-fA-F]{4})*""#,
        |r| r.slice().parse_escape_character()
            .map_err(|_| "invalid escape character".to_string())
            .and_then(|s| Ok(s.trim_matches('"').to_owned())))]
    String(String),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |r| r.slice().to_owned())]
    Var(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Error => write!(f, "Error"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Mul => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Comma => write!(f, ","),
            Token::Int(n) => write!(f, "Int({})", n),
            Token::Float(n) => write!(f, "Float({})", n),
            Token::String(s) => write!(f, "String({})", s),
            Token::Var(name) => write!(f, "Var({})", name),
        }
    }
}
