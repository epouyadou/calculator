mod lexer;
mod parser;

use lexer::Lexer;

fn main() {
    let input = "-1 + 2 * (-4 + 6) / 2"; // == -1 + 2 * 2 / 2 == -1 + 4 / 2 == -1 + 2 == 1
    let lexer = Lexer::new(input);
}
