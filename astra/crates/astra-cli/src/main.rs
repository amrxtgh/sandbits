mod lexer;
mod token;
use lexer::Lexer;

fn main() {
    println!("Astra Compiler");
    let source = " = ; ";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex();
    println!("{tokens:#?}");
}
