use crate::lexer::{
    scanner::Scanner,
    token::{DisplayTokens, Token, TokenKind},
};

#[test]
pub fn scanner_single() {
    let source = " ( ) { } [ ] , . - + ; / * : = < > ! & | % ^ ~ ? ";
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.tokenize();

    let need = vec![
        Token::without(TokenKind::LeftParen, "(".to_string(), 0),
        Token::without(TokenKind::RightParen, ")".to_string(), 0),
        Token::without(TokenKind::LeftBrace, "{".to_string(), 0),
        Token::without(TokenKind::RightBrace, "}".to_string(), 0),
        Token::without(TokenKind::LeftBracket, "[".to_string(), 0),
        Token::without(TokenKind::RightBracket, "]".to_string(), 0),
        Token::without(TokenKind::Comma, ",".to_string(), 0),
        Token::without(TokenKind::Dot, ".".to_string(), 0),
        Token::without(TokenKind::Minus, "-".to_string(), 0),
        Token::without(TokenKind::Plus, "+".to_string(), 0),
        Token::without(TokenKind::Semicolon, ";".to_string(), 0),
        Token::without(TokenKind::Slash, "/".to_string(), 0),
        Token::without(TokenKind::Star, "*".to_string(), 0),
        Token::without(TokenKind::Colon, ":".to_string(), 0),
        Token::without(TokenKind::Equal, "=".to_string(), 0),
        Token::without(TokenKind::Less, "<".to_string(), 0),
        Token::without(TokenKind::Greater, ">".to_string(), 0),
        Token::without(TokenKind::Bang, "!".to_string(), 0),
        Token::without(TokenKind::Ampersand, "&".to_string(), 0),
        Token::without(TokenKind::Pipe, "|".to_string(), 0),
        Token::without(TokenKind::Percent, "%".to_string(), 0),
        Token::without(TokenKind::Caret, "^".to_string(), 0),
        Token::without(TokenKind::Tilde, "~".to_string(), 0),
        Token::without(TokenKind::Question, "?".to_string(), 0),
    ];

    assert_eq!(need, tokens.unwrap());
}

#[test]
pub fn scanner_line() {
    let source = "let index = 51;\n\t\t index = index + 1;";
    let scanner = Scanner::new(source.to_string());
    let tokens = scanner.tokenize();

    let need = vec![
        Token::without(TokenKind::Var, "let".to_string(), 0),
        Token::without(TokenKind::Identifier, "index".to_string(), 0),
        Token::without(TokenKind::Equal, "=".to_string(), 0),
        Token::without(TokenKind::Number, "51".to_string(), 0),
        Token::without(TokenKind::Semicolon, ";".to_string(), 0),
        Token::without(TokenKind::Identifier, "index".to_string(), 1),
        Token::without(TokenKind::Equal, "=".to_string(), 1),
        Token::without(TokenKind::Identifier, "index".to_string(), 1),
        Token::without(TokenKind::Plus, "+".to_string(), 1),
        Token::without(TokenKind::Number, "1".to_string(), 1),
        Token::without(TokenKind::Semicolon, ";".to_string(), 1),
    ];

    assert_eq!(need, tokens.unwrap());
}

#[test]
pub fn print_token_and_tokens() {
    let need = vec![
        Token::without(TokenKind::LeftParen, "(".to_string(), 0),
        Token::without(TokenKind::RightParen, ")".to_string(), 0),
        Token::without(TokenKind::LeftBrace, "{".to_string(), 0),
        Token::without(TokenKind::RightBrace, "}".to_string(), 0),
        Token::without(TokenKind::LeftBracket, "[".to_string(), 0),
        Token::without(TokenKind::RightBracket, "]".to_string(), 0),
        Token::without(TokenKind::Comma, ",".to_string(), 1),
        Token::without(TokenKind::Dot, ".".to_string(), 1),
        Token::without(TokenKind::Minus, "-".to_string(), 1),
        Token::without(TokenKind::Plus, "+".to_string(), 1),
        Token::without(TokenKind::Semicolon, ";".to_string(), 1),
        Token::without(TokenKind::Slash, "/".to_string(), 1),
        Token::without(TokenKind::Star, "*".to_string(), 2),
        Token::without(TokenKind::Colon, ":".to_string(), 2),
        Token::without(TokenKind::Equal, "=".to_string(), 2),
        Token::without(TokenKind::Less, "<".to_string(), 2),
        Token::without(TokenKind::Greater, ">".to_string(), 2),
        Token::without(TokenKind::Bang, "!".to_string(), 2),
        Token::without(TokenKind::Ampersand, "&".to_string(), 3),
        Token::without(TokenKind::Pipe, "|".to_string(), 3),
        Token::without(TokenKind::Percent, "%".to_string(), 3),
        Token::without(TokenKind::Caret, "^".to_string(), 3),
        Token::without(TokenKind::Tilde, "~".to_string(), 3),
        Token::without(TokenKind::Question, "?".to_string(), 3),
    ];

    println!("{}", DisplayTokens(&need));
}
