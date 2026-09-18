use crate::lexer::{
    scanner::Scanner,
    token::{Token, TokenKind},
};

#[test]
pub fn scanner_single() {
    let source = " ( ) { } [ ] , . - + ; / * : = < > ! & | % ^ ~ ? ";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.tokenize();

    let need = vec![
        Token::without(TokenKind::LeftParen, "(", 1..2),
        Token::without(TokenKind::RightParen, ")", 3..4),
        Token::without(TokenKind::LeftBrace, "{", 5..6),
        Token::without(TokenKind::RightBrace, "}", 7..8),
        Token::without(TokenKind::LeftBracket, "[", 9..10),
        Token::without(TokenKind::RightBracket, "]", 11..12),
        Token::without(TokenKind::Comma, ",", 13..14),
        Token::without(TokenKind::Dot, ".", 15..16),
        Token::without(TokenKind::Minus, "-", 17..18),
        Token::without(TokenKind::Plus, "+", 19..20),
        Token::without(TokenKind::Semicolon, ";", 21..22),
        Token::without(TokenKind::Slash, "/", 23..24),
        Token::without(TokenKind::Star, "*", 25..26),
        Token::without(TokenKind::Colon, ":", 27..28),
        Token::without(TokenKind::Equal, "=", 29..30),
        Token::without(TokenKind::Less, "<", 31..32),
        Token::without(TokenKind::Greater, ">", 33..34),
        Token::without(TokenKind::Bang, "!", 35..36),
        Token::without(TokenKind::Ampersand, "&", 37..38),
        Token::without(TokenKind::Pipe, "|", 39..40),
        Token::without(TokenKind::Percent, "%", 41..42),
        Token::without(TokenKind::Caret, "^", 43..44),
        Token::without(TokenKind::Tilde, "~", 45..46),
        Token::without(TokenKind::Question, "?", 47..48),
    ];

    assert_eq!(need, tokens);
}

#[test]
pub fn scanner_line() {
    let source = "let index = 51;\n\t\t index = index + 1;";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.tokenize();

    let need = vec![
        Token::without(TokenKind::Var, "let", 0..3),
        Token::without(TokenKind::Identifier, "index", 4..9),
        Token::without(TokenKind::Equal, "=", 10..11),
        Token::without(TokenKind::Number, "51", 12..14),
        Token::without(TokenKind::Semicolon, ";", 14..15),
        Token::without(TokenKind::Identifier, "index", 19..24),
        Token::without(TokenKind::Equal, "=", 25..26),
        Token::without(TokenKind::Identifier, "index", 27..32),
        Token::without(TokenKind::Plus, "+", 33..34),
        Token::without(TokenKind::Number, "1", 35..36),
        Token::without(TokenKind::Semicolon, ";", 36..37),
    ];

    assert_eq!(need, tokens);
}
