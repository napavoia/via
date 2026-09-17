use std::fmt::Display;

use regex::regex;

use crate::{lexer::token::TokenKind::Identifier, option_match};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lex: String,
    pub val: Value,
    pub line: usize,
}

impl Token {
    #[inline]
    pub fn semicolon(line: usize) -> Self {
        Self::without(TokenKind::Semicolon, String::from(";"), line)
    }

    #[inline]
    pub fn without(kind: TokenKind, lex: String, line: usize) -> Self {
        Self::with(kind, lex, Value::None, line)
    }

    #[inline]
    pub fn with(kind: TokenKind, lex: String, val: Value, line: usize) -> Self {
        Self {
            kind,
            lex,
            val,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.kind.has_value() {
            write!(
                f,
                "[T:{}] {:?}('{}' - '{}')",
                self.line, self.kind, self.lex, self.val
            )
        } else {
            write!(f, "[T:{}] {:?}('{}')", self.line, self.kind, self.lex)
        }
    }
}

pub trait Tokenized {
    fn token(&mut self, line: usize) -> Option<Token>;
}

impl Tokenized for String {
    fn token(&mut self, line: usize) -> Option<Token> {
        fn single(s: &String) -> Option<TokenKind> {
            let s = s.as_str();

            if regex!(r"^-?\d+$").is_match(s) {
                return Some(TokenKind::Number);
            }

            return option_match!(s
                "(" => TokenKind::LeftParen,
                ")" => TokenKind::RightParen,
                "{" => TokenKind::LeftBrace,
                "}" => TokenKind::RightBrace,
                "," => TokenKind::Comma,
                "." => TokenKind::Dot,
                "-" => TokenKind::Minus,
                "+" => TokenKind::Plus,
                "/" => TokenKind::Slash,
                "*" => TokenKind::Star,
                ":" => TokenKind::Colon,
                "=" => TokenKind::Equal,
                "<" => TokenKind::Less,
                ">" => TokenKind::Greater,
                "!" => TokenKind::Bang,
                "&" => TokenKind::Ampersand,
                "|" => TokenKind::Pipe,
                "%" => TokenKind::Percent,
                "^" => TokenKind::Caret,
                "~" => TokenKind::Tilde,
                "?" => TokenKind::Question,
                "[" => TokenKind::LeftBracket,
                "]" => TokenKind::RightBracket
            );
        }

        fn mult(s: &String) -> Option<TokenKind> {
            let s = s.as_str();

            if regex!(r"^-?\d+$").is_match(s) {
                return Some(TokenKind::Number);
            } else if regex!(r"^-?(\d+\.\d*|\.\d+|\d+)([eE][+-]?\d+)?$").is_match(s) {
                return Some(TokenKind::Float);
            } else if regex!(r#"^"([^"\\]|\\.)*"$"#).is_match(s) {
                return Some(TokenKind::String);
            }

            return option_match!(s
                "!=" => TokenKind::BangEqual,
                "==" => TokenKind::EqualEqual,
                ">=" => TokenKind::GreaterEqual,
                "<=" => TokenKind::LessEqual,
                "and" => TokenKind::And,
                "class" => TokenKind::Class,
                "else" => TokenKind::Else,
                "false" => TokenKind::False,
                "fn" => TokenKind::Fn,
                "for" => TokenKind::For,
                "if" => TokenKind::If,
                "null" => TokenKind::Null,
                "or" => TokenKind::Or,
                "return" => TokenKind::Return,
                "self" => TokenKind::This,
                "true" => TokenKind::True,
                "let" => TokenKind::Var,
                "while" => TokenKind::While;
                Some(Identifier)
            );
        }

        let out = if self.len() == 1 {
            let kind = single(self);
            if kind.is_none() {
                return None;
            }

            Some(Token::without(kind.unwrap(), self.clone(), line))
        } else {
            let kind = mult(self);
            if kind.is_none() {
                return None;
            }

            Some(Token::without(kind.unwrap(), self.clone(), line))
        };

        self.clear();
        out
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    None,
    String(String),
    Float(f32),
    Integer(i32),
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Self::String(value.to_string())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::None => Ok(()),
            Value::String(s) => write!(f, "{s}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::Integer(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum TokenKind {
    LeftParen,    //  (
    RightParen,   //  )
    LeftBrace,    //  {
    RightBrace,   //  }
    LeftBracket,  //  [
    RightBracket, //  ]
    Comma,        //  ,
    Dot,          //  .
    Minus,        //  -
    Plus,         //  +
    Semicolon,    //  ;
    Slash,        //  /
    Star,         //  *
    Ampersand,    //  &
    Pipe,         //  |
    Percent,      //  %
    Caret,        //  ^
    Tilde,        //  ~
    Question,     //  ?
    Colon,        //  :

    Bang,         //  !
    BangEqual,    //  !=
    Equal,        //  =
    EqualEqual,   //  ==
    Greater,      //  >
    GreaterEqual, //  >=
    Less,         //  <
    LessEqual,    //  <=

    Identifier, //
    String,     //   ""
    Number,     //   1234
    Float,      //   12.34

    And,    // and
    Class,  // class
    Else,   // else
    False,  // false
    Fn,     // fn
    For,    // for
    If,     // if
    Null,   // null
    Or,     // or
    Return, // return
    This,   // self
    True,   // true
    Var,    // let
    While,  // while

    EOF,
    NONE,
}

impl TokenKind {
    pub fn has_value(&self) -> bool {
        match self {
            Self::String => true,
            Self::Float => true,
            Self::Number => true,
            Self::Null => true,
            _ => false,
        }
    }
}

pub struct DisplayTokens<'a>(pub &'a [Token]);

impl Display for DisplayTokens<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line = (0, false);
        write!(f, "=[  ")?;
        for (i, t) in self.0.iter().enumerate() {
            if !line.1 {
                line.1 = true;
                line.0 = t.line;
            }

            if t.line > line.0 {
                line.0 = t.line;
                write!(f, "  ]=")?;
                write!(f, "\n")?;
                write!(f, "=[  ")?;
            } else if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", t)?;
        }
        write!(f, "  ]=")
    }
}
