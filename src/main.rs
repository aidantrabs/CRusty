use std::env;
use std::fs;
use std::process;

mod lexer;

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

    let input = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            process::exit(1);
        }
    };

    let tokens = lexer::lex(&input);
    println!("{:?}", tokens);
}