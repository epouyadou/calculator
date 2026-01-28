mod lexer;
mod parser;
mod interpreter;

use std::io;
use lexer::Lexer;
use parser::Parser;
use parser::error::ParseError;
use interpreter::evaluate;

fn main() {
    loop {
        let mut user_input = String::new();

        println!("\nPlease enter expression: ");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input == "exit" {
            break;
        }

        let lexer = Lexer::new(user_input.as_str());
        let mut parser = Parser::new(lexer);

        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(e) => {
                match e {
                    ParseError::UnexpectedToken { expected, found } => println!("Unexpected token (expected: '{:?}', found: '{:?}')", expected, found),
                    ParseError::MissingClosingParenthesis => println!("Missing closing parenthesis"),
                    ParseError::InvalidExpression => println!("Invalid expression"),
                    ParseError::InvalidToken { token, char } => println!("Invalid token: {:?} at char: {}", token, char),
                    _ => {}
                }
                continue;
            },
        };

        match evaluate(&ast, 0) {
            Ok(result) => {
                println!("Result: {}", result);
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

    }
}
