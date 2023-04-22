use std::env;
use std::process;
use std::fs::*;
use std::io::prelude::*;

mod lexer;
use lexer::Lexer;
// mod parser;

/*
    @Description: Main function
    @Params: None
    @Returns: None
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    
    let mut buffer1 = String::new();
    let mut buffer2 = String::new();
    let mut file = File::open(filename).expect("Unable to open file");

    file.read_to_string(&mut buffer2).expect("Unable to read file");
    buffer1.push_str(&buffer2);

    let tokens = Lexer::get_next_token(&mut buffer1);

    println!("{:#?}", tokens);
}