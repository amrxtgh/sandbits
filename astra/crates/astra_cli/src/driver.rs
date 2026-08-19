use crate::input::read_source;
use astra_lexer::lexer::Lexer;
use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: astra <files>");
        return;
    }
    let filename = &args[1];

    // input does the file handling
    let source = match read_source(filename) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("failed to read {filename}: {error}");
            return;
        }
    };
    println!("{source}");

    // returns char of tokens and pos
    let mut lexer = Lexer::new(&source);
    // do the tokenization
    // tokenize return Result
    match lexer.tokenize() {
        Ok(token) => {
            println!("{:#?}", token);
        }
        Err(error) => {
            eprintln!("lexer error: {error}");
        }
    }
}
