use std::fs;
use std::fs::File;
use std::io::prelude::*;

/*
     @Description: Enum of all possible tokens
     @Params: None
     @Returns: None
*/
#[derive(Debug, PartialEq)]
pub enum TokenTypes {
     Def,
     Type(String),
     Ident(String),
     LParen,
     RParen,
     Comma,
     Semicolon,
     Assign,
     Plus,
     PlusAssign,
     Minus,
     MinusEqual,
     Asterisk,
     AsteriskEqual,
     Divide,
     DivideEqual,
     Modulo,
     ModuloEqual,
     If,
     Then,
     Else,
     Fi,
     While,
     Do,
     Od,
     Print,
     Return,
     IntegerLiteral(i32),
     DoubleLiteral(f64),
     Or,
     And,
     Not,
     Less,
     Greater,
     Equal,
     LessEqual,
     GreaterEqual,
     NotEqual,
     LBracket,
     RBracket,
     Error,
}

/*
     @Description: Struct for tokens
     @Params: None
     @Returns: None
*/
#[derive(Debug)]
pub struct Token {
     pub token_type: TokenTypes,
     pub line_number: usize,
     pub column_number: usize,
     pub value: String,
}

/* 
     @Description: Lexical Analyzer
     @Param: input - String
     @Return: Token on different line with line number
*/
pub fn get_next_token(input: &str) -> Result<Vec<Token>, String> {
     let mut tokens = Vec::new();
     let mut chars = input.chars().peekable();
     let mut line_number = 1;
     let mut column_number = 0;

     while let Some(c) = chars.next() {
          if !(c == ' ' || c == '\t' || c == '\r') {
               column_number += 1;

               if c == '\n' {
                    column_number = 0;
               }
          }
          
          match c {
               '\n' => line_number += 1,
               ' ' | '\t' | '\r' => continue,
               'a'..='z' | 'A'..='Z' => {
                    let mut ident = String::new();
                    ident.push(c);
                    while let Some(&c) = chars.peek() {
                         match c {
                              'a'..='z' | 'A'..='Z' | '0'..='9' => {
                                   ident.push(c);
                                   chars.next();
                              }
                              _ => break,
                         }
                    }

                    let token_type = match ident.as_str() {
                         "def" => TokenTypes::Def,
                         "type" => TokenTypes::Type(ident.clone()),
                         "if" => TokenTypes::If,
                         "then" => TokenTypes::Then,
                         "else" => TokenTypes::Else,
                         "fi" => TokenTypes::Fi,
                         "while" => TokenTypes::While,
                         "do" => TokenTypes::Do,
                         "od" => TokenTypes::Od,
                         "print" => TokenTypes::Print,
                         "return" => TokenTypes::Return,
                         "or" => TokenTypes::Or,
                         "and" => TokenTypes::And,
                         "not" => TokenTypes::Not,
                         "int" => TokenTypes::Type(ident.clone()),
                         "double" => TokenTypes::Type(ident.clone()),
                         "bool" => TokenTypes::Type(ident.clone()),
                         "string" => TokenTypes::Type(ident.clone()),
                         "void" => TokenTypes::Type(ident.clone()),
                         _ => TokenTypes::Ident(ident.clone()),
                    };
                    tokens.push(Token {
                         token_type,
                         line_number,
                         column_number,
                         value: ident.as_str().to_string(),
                    });
               }

               '0'..='9' => {
                    let mut number = String::new();
                    number.push(c);
                    while let Some(&c) = chars.peek() {
                         match c {
                              '0'..='9' => {
                                   number.push(c);
                                   chars.next();
                              }
                              _ => break,
                         }
                    }

                    if let Some(&'.') = chars.peek() {
                         number.push('.');
                         chars.next();
                         while let Some(&c) = chars.peek() {
                              match c {
                                   '0'..='9' => {
                                        number.push(c);
                                        chars.next();
                                   }
                                   _ => break,
                              }
                         }
                         tokens.push(Token {
                              token_type: TokenTypes::DoubleLiteral(
                                   number.parse().expect("Unable to parse double"),
                              ),
                              line_number,
                              column_number,
                              value: number.clone(),
                         });
                    } else {
                         tokens.push(Token {
                              token_type: TokenTypes::IntegerLiteral(
                                   number.parse().expect("Unable to parse integer"),
                              ),
                              line_number,
                              column_number,
                              value: number.clone(),
                         });
                    }

                    if let Some(&c) = chars.peek() {
                         if c.is_alphabetic()  {
                              tokens.push(Token {
                                   token_type: TokenTypes::Error,
                                   line_number,
                                   column_number: column_number + 1,
                                   value: String::from("Invalid token - ".to_owned() + &c.to_string()),
                              });
                         }
                    }
               }

               '(' => tokens.push(Token {
                    token_type: TokenTypes::LParen,
                    line_number,
                    column_number,
                    value: String::from("("),
               }),

               ')' => tokens.push(Token {
                    token_type: TokenTypes::RParen,
                    line_number,
                    column_number,
                    value: String::from(")"),
               }),

               ',' => tokens.push(Token {
                    token_type: TokenTypes::Comma,
                    line_number,
                    column_number,
                    value: String::from(","),
               }),

               ';' => tokens.push(Token {
                    token_type: TokenTypes::Semicolon,
                    line_number,
                    column_number,
                    value: String::from(";"),
               }),

               '=' => tokens.push(Token {
                    token_type: TokenTypes::Assign,
                    line_number,
                    column_number,
                    value: String::from("="),
               }),

               '+' => {
                    if let Some(&'=') = chars.peek() {
                         chars.next();
                         tokens.push(Token {
                              token_type: TokenTypes::PlusAssign,
                              line_number,
                              column_number,
                              value: String::from("+="),
                         });
                    } else {
                         tokens.push(Token {
                              token_type: TokenTypes::Plus,
                              line_number,
                              column_number,
                              value: String::from("+"),
                         });
                    }
               }

               '-' => {
                    if let Some(&'=') = chars.peek() {
                         chars.next();
                         tokens.push(Token {
                              token_type: TokenTypes::MinusEqual,
                              line_number,
                              column_number,
                              value: String::from("-="),
                         });
                    } else {
                         tokens.push(Token {
                              token_type: TokenTypes::Minus,
                              line_number,
                              column_number,
                              value: String::from("-"),
                         });
                    }
               }

               '*' => {
                    if let Some(&'=') = chars.peek() {
                         chars.next();
                         tokens.push(Token {
                              token_type: TokenTypes::AsteriskEqual,
                              line_number,
                              column_number,
                              value: String::from("*="),
                         });
                    } else {
                         tokens.push(Token {
                              token_type: TokenTypes::Asterisk,
                              line_number,
                              column_number,
                              value: String::from("*"),
                         });
                    }
               }

               '/' => {
                    if let Some(&'=') = chars.peek() {
                         chars.next();
                         tokens.push(Token {
                              token_type: TokenTypes::DivideEqual,
                              line_number,
                              column_number,
                              value: String::from("/="),
                         });
                    } else {
                         tokens.push(Token {
                              token_type: TokenTypes::Divide,
                              line_number,
                              column_number,
                              value: String::from("/"),
                         });
                    }
               }

               '%' => {
                    if let Some(&'=') = chars.peek() {
                         chars.next();
                         tokens.push(Token {
                              token_type: TokenTypes::ModuloEqual,
                              line_number,
                              column_number,
                              value: String::from("%="),
                         });
                    } else {
                         tokens.push(Token {
                              token_type: TokenTypes::Modulo,
                              line_number,
                              column_number,
                              value: String::from("%"),
                         });
                    }
               }

               '<' => {
                    if let Some(&'=') = chars.peek() {
                         chars.next();
                         tokens.push(Token {
                              token_type: TokenTypes::LessEqual,
                              line_number,
                              column_number,
                              value: String::from("<="),
                         });
                    } 

                    if let Some(&'>') = chars.peek() {
                         chars.next();
                         tokens.push(Token {
                              token_type: TokenTypes::NotEqual,
                              line_number,
                              column_number,
                              value: String::from("<>"),
                         });
                    } else {
                         tokens.push(Token {
                              token_type: TokenTypes::Less,
                              line_number,
                              column_number,
                              value: String::from("<"),
                         });
                    }
               }

               '>' => {
                    if let Some(&'=') = chars.peek() {
                         chars.next();
                         tokens.push(Token {
                              token_type: TokenTypes::GreaterEqual,
                              line_number,
                              column_number,
                              value: String::from(">="),
                         });
                    } else {
                         tokens.push(Token {
                              token_type: TokenTypes::Greater,
                              line_number,
                              column_number,
                              value: String::from(">"),
                         });
                    }
               }

               '!' => {
                    if let Some(&'=') = chars.peek() {
                         chars.next();
                         tokens.push(Token {
                              token_type: TokenTypes::NotEqual,
                              line_number,
                              column_number,
                              value: String::from("!="),

                         });
                    } else {
                         tokens.push(Token {
                              token_type: TokenTypes::Not,
                              line_number,
                              column_number,
                              value: String::from("!"),
                         });
                    }
               }

              _ => {
                   tokens.push(Token {
                        token_type: TokenTypes::Error,
                        line_number,
                        column_number,
                        value: String::from("Invalid token - ".to_owned() + &c.to_string()),
                   });
              }
          }
     }
     Ok(tokens)
}

/* 
     @Description: Runs the get_next_token function on an input file
     @Params: file_name: String
     @Returns: Result<Vec<Token>, String>
*/
pub fn run_lexical_analysis(file_name: String) {
     let mut error_file = File::create("error.log").expect("Unable to create file");
     let mut valid_file = File::create("valid.log").expect("Unable to create file");
 
     let input = fs::read_to_string(file_name).expect("Unable to read file");
 
     writeln!(valid_file,
         "{0: <15} | {1: <15} | {2: <15} | {3: <15}",
         "Token Type", "Line Number", "Column Number", "Value"
     ).expect("Unable to write to file");
 
     match get_next_token(&input) {
         Ok(tokens) => {
             for token in tokens {
                 if token.token_type == TokenTypes::Error {
                     writeln!(error_file, "{:?} | {:?} | {:?} | {:?}", 
                         token.token_type, token.line_number, token.column_number, token.value
                     ).expect("Unable to write to file");
                     continue;
                 }
 
                 writeln!(valid_file, "{:?} | {:?} | {:?} | {:?}", 
                     token.token_type, token.line_number, token.column_number, token.value
                 ).expect("Unable to write to file");
             }
         }
         Err(e) => {
             writeln!(error_file, "{}", e).expect("Unable to write to file");
         }
     }   
 
     let tokens = get_next_token(&input);
     println!("{:?}", tokens.unwrap());
}
