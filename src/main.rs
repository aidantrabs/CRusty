use std::env;
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
    lexer::run_lexical_analysis(filename.to_string());
}