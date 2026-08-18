use astra_lexer::lexer::Lexer;
use std::{env, fs};
fn main() {
    println!("Astra Compiler");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: astra <files>");
        return;
    }
    let filename = &args[1];

    let source = match fs::read_to_string(filename) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("failed to read {filename}: {error}");
            return;
        }
    };
    println!("{source}");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.lex();
    println!("{tokens:#?}");
}
