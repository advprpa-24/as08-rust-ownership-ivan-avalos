use std::io;
use std::io::Write;

use lc::eval::*;
use lc::parser::*;

/// Driver code to run the lambda calculus evaluator.
// NOTE!! the parser I copied is a bit shitty, so all function
// applications must be surrounded by parentheses.
//
// EXAMPLE: ((λx. x) (λy. y)) instead of (λx. x) (λy. y)
fn main() {
    loop {
        let mut input = String::new();
        print!("Introduce a lambda term: ");
        io::stdout().flush().expect("Could not flush buffer");
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match parse(&input.trim()) {
                    Ok(t) => {
                        println!("Original term: {}", t);
                        let result = eval(&t);
                        println!("Evaluated term: {result}")
                    },
                    Err(error) => {
                        println!("Parse error: {error}")
                    },
                }
            },
            Err(error) => {
                println!("error: {error}")
            },
        }
    }
}
