use monkey_lang::{
    ast::{Expression, Node, Statement},
    lexer::Lexer,
    parser::Parser,
};

struct ExpextedIdentifier(String);

impl ExpextedIdentifier {
    fn expexted_identifier(&self) -> &str {
        &self.0
    }
}

macro_rules! ei {
    ($expexted_identifier:expr) => {
        ExpextedIdentifier(String::from($expexted_identifier))
    };
}

const PARSER_TEST_INPUT: &str = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;

#[test]
fn test_let_statements() {
    let input = String::from(PARSER_TEST_INPUT);
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    assert!(program.is_some());

    if let Some(ref program) = program {
        assert_eq!(program.statements().len(), 3);
    }

    let tests = [ei!("x"), ei!("y"), ei!("foobar")];

    if let Some(program) = program {
        for (i, tt) in tests.iter().enumerate() {
            if let Some(stmt) = program.statements().get(i) {
                test_let_statement(stmt, tt.expexted_identifier());
            }
        }
    }
}

fn test_let_statement(s: &Statement, name: &str) {
    assert_eq!(s.token_literal(), name);
    assert!(
        match s {
            Statement::LetStatement { .. } => true,
            _ => false,
        },
        "s not a &Statement::LetStatement"
    );
    assert!(match s {
        Statement::LetStatement {
            token: _,
            name: identifier @ Expression::Identifier { token: _, value },
            ..
        } if value == name && identifier.token_literal() == name => true,
        _ => false,
    });
}
