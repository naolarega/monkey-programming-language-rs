use monkey_lang::{
    lexer::Lexer,
    token::TokenType::{self, *},
};

struct ExpectedTokenTypeLiteralPair {
    expected_type: TokenType,
    expected_literal: String,
}

/// # ettlp (Expected token type literal pair)
///
/// ```
/// ettlp!(Let, "let");
/// ```
/// Expands to
/// ```
/// ExpectedTokenTypeLiteralPair {
///     expected_type: Let,
///     expected_literal: String::from("let"),
/// }
/// ```
macro_rules! ettlp {
    ($expected_type:ident, $expected_literal:expr) => {
        ExpectedTokenTypeLiteralPair {
            expected_type: $expected_type,
            expected_literal: String::from($expected_literal),
        }
    };
}

const LEXER_INPUT: &str = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;

#[test]
fn test_next_token() {
    let input = String::from(LEXER_INPUT);
    let tests = [
        ettlp!(Let, "let"),
        ettlp!(Ident, "five"),
        ettlp!(Assign, "="),
        ettlp!(Int, "5"),
        ettlp!(Semicolon, ";"),
        ettlp!(Let, "let"),
        ettlp!(Ident, "ten"),
        ettlp!(Assign, "="),
        ettlp!(Int, "10"),
        ettlp!(Semicolon, ";"),
        ettlp!(Let, "let"),
        ettlp!(Ident, "add"),
        ettlp!(Assign, "="),
        ettlp!(Function, "fn"),
        ettlp!(Lparen, "("),
        ettlp!(Ident, "x"),
        ettlp!(Comma, ","),
        ettlp!(Ident, "y"),
        ettlp!(Rparen, ")"),
        ettlp!(Lbrace, "{"),
        ettlp!(Ident, "x"),
        ettlp!(Plus, "+"),
        ettlp!(Ident, "y"),
        ettlp!(Semicolon, ";"),
        ettlp!(Rbrace, "}"),
        ettlp!(Semicolon, ";"),
        ettlp!(Let, "let"),
        ettlp!(Ident, "result"),
        ettlp!(Assign, "="),
        ettlp!(Ident, "add"),
        ettlp!(Lparen, "("),
        ettlp!(Ident, "five"),
        ettlp!(Comma, ","),
        ettlp!(Ident, "ten"),
        ettlp!(Rparen, ")"),
        ettlp!(Semicolon, ";"),
        ettlp!(Bang, "!"),
        ettlp!(Minus, "-"),
        ettlp!(Slash, "/"),
        ettlp!(Asterisk, "*"),
        ettlp!(Int, "5"),
        ettlp!(Semicolon, ";"),
        ettlp!(Int, "5"),
        ettlp!(Lt, "<"),
        ettlp!(Int, "10"),
        ettlp!(Gt, ">"),
        ettlp!(Int, "5"),
        ettlp!(Semicolon, ";"),
        ettlp!(If, "if"),
        ettlp!(Lparen, "("),
        ettlp!(Int, "5"),
        ettlp!(Lt, "<"),
        ettlp!(Int, "10"),
        ettlp!(Rparen, ")"),
        ettlp!(Lbrace, "{"),
        ettlp!(Return, "return"),
        ettlp!(True, "true"),
        ettlp!(Semicolon, ";"),
        ettlp!(Rbrace, "}"),
        ettlp!(Else, "else"),
        ettlp!(Lbrace, "{"),
        ettlp!(Return, "return"),
        ettlp!(False, "false"),
        ettlp!(Semicolon, ";"),
        ettlp!(Rbrace, "}"),
        ettlp!(Int, "10"),
        ettlp!(Eq, "=="),
        ettlp!(Int, "10"),
        ettlp!(Semicolon, ";"),
        ettlp!(Int, "10"),
        ettlp!(NotEq, "!="),
        ettlp!(Int, "9"),
        ettlp!(Semicolon, ";"),
        ettlp!(Eof, "\0"),
    ];
    let mut lexer = Lexer::new(input);

    for expected_token in tests.iter() {
        let token = lexer.next_token();

        assert!(token.token_type() == expected_token.expected_type);
        assert!(token.literal() == expected_token.expected_literal);
    }
}
