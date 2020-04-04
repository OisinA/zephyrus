use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use zephyrus::lexer;
use zephyrus::token::Token;
use zephyrus::parser;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No file specified.");
        exit(1);
    }

    let contents = fs::read_to_string(args[1].as_str())
        .expect("Something went wrong reading the file :(");

    let mut lex = lexer::Lexer::new(&contents);
    let mut parser = parser::Parser::new(lex);
    let output = match parser.parse() {
        Ok(output) => output,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    let path = Path::new(args[2].as_str());
    let mut file = match File::create(&path) {
        Err(err) => panic!("couldn't create file"),
        Ok(file) => file,
    };

    match(file.write_all(output.as_bytes())) {
        Err(err) => panic!("coudln't write to file"),
        Ok(_) => println!("wrote to out.html"),
    }
}
