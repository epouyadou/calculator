# Calculator

A high-performance mathematical calculator written in Rust. It transforms raw text into an Abstract Syntax Tree (AST) for precise evaluation, supporting standard arithmetic and implicit operations.

## Features
- **Efficient Lexing**: Operates directly on `&[u8]` byte slices for zero-copy performance.
- **Implicit Multiplication**: Supports algebraic-style notation like `2(3 + 4)`.
- **Unary Operators**: Full support for negative signs (e.g., `-5`).
- **Traceable Evaluation**: Recursive visualization of the calculation steps.

## Getting Started

Ensure you have [Rust](https://www.rust-lang.org/) installed.

1. Clone the repository:
   ```bash
   git clone [https://github.com/epouyadou/calculator.git](https://github.com/epouyadou/calculator.git)
   cd calculator
   ```

2. Run the program:
   ```bash
   cargo run
   ```
   
## Testing

Run the unit tests for the lexer and parser:

```bash
cargo test
```

## Example Output (Verbose Mode)

If you run the code with the `RUST_LOG=debug` env variable, you'll be able to see the following output.
```
-> Binary Star
  -> Number: 2
  -> Grouping ( )
    -> Binary Plus
      -> Number: 3
      -> Number: 4
   = Result: 14
```

## GRAMMAR
```text
// Top-level entry point
expression     → addition ;

// Binary operators (lowest priority first)
addition       → multiplication ( ( "-" | "+" ) multiplication )* ;

// Handles standard *, / and implicit multiplication like 2(3)
multiplication → unary ( ( "/" | "*" | "(" ) unary )* ;

// Unary operators (right-associative)
unary          → ( "-" | "+" ) unary 
               | primary ;

// Highest priority
primary        → NUMBER 
               | "(" expression ")" ;

// Terminal tokens
NUMBER         → DIGIT+ ( "." DIGIT+ )? ;
DIGIT          → "0" ... "9" ;
```
