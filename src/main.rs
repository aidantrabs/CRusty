use std::env;
use std::fs;
use std::process;
use std::fs::File;
use std::io::prelude::*;

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
    let mut error_file = File::create("error.log").expect("Unable to create file");
    let mut valid_file = File::create("valid.log").expect("Unable to create file");

    let input = fs::read_to_string(filename).expect("Unable to read file");

    match lexer::get_next_token(&input) {
        Ok(tokens) => {
            for token in tokens {
                writeln!(valid_file, "{:?}", token).expect("Unable to write to file");
            }
        }
        Err(e) => {
            writeln!(error_file, "{}", e).expect("Unable to write to file");
        }
    }

    let tokens = lexer::get_next_token(&input);
    println!("{:?}", tokens);
}