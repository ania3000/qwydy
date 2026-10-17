mod keywords;
mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let tokens_only = args.get(1).is_some_and(|arg| arg == "--tokens");

    let path = if tokens_only {
        args.get(2).map(String::as_str)
    } else {
        args.get(1).map(String::as_str)
    };

    let source = match path {
        Some(path) => match fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("Рӕдыд {path} кӕсгӕйӕ: {error}");
                std::process::exit(1);
            }
        },
        None => "20 + 22 * 2".to_string(),
    };

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    if tokens_only {
        println!("Токентӕ:\n");

        for token in &tokens {
            println!(
                "{:?} (строка {}, столбец {})",
                token.kind, token.line, token.column
            );
        }

        return;
    }

    let mut parser = parser::Parser::new(tokens);

    match parser.parse() {
        Ok(()) => println!("Цӕттӕ."),
        Err(error) => {
            eprintln!("Рӕдыд: {error}");
            std::process::exit(1);
        }
    }
}
