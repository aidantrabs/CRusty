use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use comfy_table::Table;

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
     Eof,
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
     @Description: String representation of the token types
     @Params: None
     @Returns: None
*/
impl fmt::Display for TokenTypes {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          match *self {
               TokenTypes::Def => write!(f, "Def"),
               TokenTypes::Type(ref s) => write!(f, "Type({})", s),
               TokenTypes::Ident(ref s) => write!(f, "Ident({})", s),
               TokenTypes::LParen => write!(f, "LParen"),
               TokenTypes::RParen => write!(f, "RParen"),
               TokenTypes::Comma => write!(f, "Comma"),
               TokenTypes::Semicolon => write!(f, "Semicolon"),
               TokenTypes::Assign => write!(f, "Assign"),
               TokenTypes::Plus => write!(f, "Plus"),
               TokenTypes::PlusAssign => write!(f, "PlusAssign"),
               TokenTypes::Minus => write!(f, "Minus"),
               TokenTypes::MinusEqual => write!(f, "MinusEqual"),
               TokenTypes::Asterisk => write!(f, "Asterisk"),
               TokenTypes::AsteriskEqual => write!(f, "AsteriskEqual"),
               TokenTypes::Divide => write!(f, "Divide"),
               TokenTypes::DivideEqual => write!(f, "DivideEqual"),
               TokenTypes::Modulo => write!(f, "Modulo"),
               TokenTypes::ModuloEqual => write!(f, "ModuloEqual"),
               TokenTypes::If => write!(f, "If"),
               TokenTypes::Then => write!(f, "Then"),
               TokenTypes::Else => write!(f, "Else"),
               TokenTypes::Fi => write!(f, "Fi"),
               TokenTypes::While => write!(f, "While"),
               TokenTypes::Do => write!(f, "Do"),
               TokenTypes::Od => write!(f, "Od"),
               TokenTypes::Print => write!(f, "Print"),
               TokenTypes::Return => write!(f, "Return"),
               TokenTypes::Eof => write!(f, "Eof"),
               TokenTypes::IntegerLiteral(ref i) => write!(f, "IntegerLiteral({})", i),
               TokenTypes::DoubleLiteral(ref d) => write!(f, "DoubleLiteral({})", d),
               TokenTypes::Or => write!(f, "Or"),
               TokenTypes::And => write!(f, "And"),
               TokenTypes::Not => write!(f, "Not"),
               TokenTypes::Less => write!(f, "Less"),
               TokenTypes::Greater => write!(f, "Greater"),
               TokenTypes::Equal => write!(f, "Equal"),
               TokenTypes::LessEqual => write!(f, "LessEqual"),
               TokenTypes::GreaterEqual => write!(f, "GreaterEqual"),
               TokenTypes::NotEqual => write!(f, "NotEqual"),
               TokenTypes::LBracket => write!(f, "LBracket"),
               TokenTypes::RBracket => write!(f, "RBracket"),
               TokenTypes::Error => write!(f, "Error"),
          }
     }
}

/*
     @Description: Struct for tokens
     @Params: None
     @Returns: None
*/
#[derive(Debug)]
pub struct Token {
     pub token_type: TokenTypes,
     pub lexeme: String,
     pub line_number: usize,
     pub column_number: usize,
}

/*
     @Description: Lexer struct
     @Params: None
     @Returns: None
*/
pub struct Lexer {
     pub tokens: Vec<Token>,
     pub line_number: usize,
     pub column_number: usize,
     pub lexeme: String,
}

/*
     @Description: Lexer constructor
     @Params: None
     @Returns: None
*/
impl Lexer {
     pub fn get_next_token(input: &str) -> Result<Vec<Token>, String> {
          let mut tokens = Vec::new();
          let mut chars = input.chars().peekable();
          let mut line_number = 1;
          let mut column_number = 0;

          let mut error_file = File::create("data/output/lexer-error.log").expect("Unable to create file");
          let mut valid_file = File::create("data/output/lexer-valid.log").expect("Unable to create file");
     
          let mut valid_table = Table::new();
          let mut error_table = Table::new();
     
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
                              lexeme: ident.as_str().to_string(),
                              line_number,
                              column_number,
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
                                   lexeme: number.clone(),
                                   line_number,
                                   column_number,
                              });
                         } else {
                              tokens.push(Token {
                                   token_type: TokenTypes::IntegerLiteral(
                                        number.parse().expect("Unable to parse integer"),
                                   ),
                                   lexeme: number.clone(),
                                   line_number,
                                   column_number,
                              });
                         }
     
                         if let Some(&c) = chars.peek() {
                              if c.is_alphabetic()  {
                                   tokens.push(Token {
                                        token_type: TokenTypes::Error,
                                        lexeme: String::from("".to_owned() + &c.to_string()),
                                        line_number,
                                        column_number: column_number + 1,
                                   });
                              }
                         }
                    }
     
                    '(' => tokens.push(Token {
                         token_type: TokenTypes::LParen,
                         lexeme: String::from("("),
                         line_number,
                         column_number,
                    }),
     
                    ')' => tokens.push(Token {
                         token_type: TokenTypes::RParen,
                         lexeme: String::from(")"),
                         line_number,
                         column_number,
                    }),
     
                    '[' => tokens.push(Token {
                         token_type: TokenTypes::LBracket,
                         lexeme: String::from("["),
                         line_number,
                         column_number,
                    }),
     
                    ']' => tokens.push(Token {
                         token_type: TokenTypes::RBracket,
                         lexeme: String::from("]"),
                         line_number,
                         column_number,
                    }),
     
                    ',' => tokens.push(Token {
                         token_type: TokenTypes::Comma,
                         lexeme: String::from(","),
                         line_number,
                         column_number,
                    }),
     
                    ';' => tokens.push(Token {
                         token_type: TokenTypes::Semicolon,
                         lexeme: String::from(";"),
                         line_number,
                         column_number,
                    }),
     
                    '=' => {
                         if let Some(&'=') = chars.peek() {
                              chars.next();
                              tokens.push(Token {
                                   token_type: TokenTypes::Equal,
                                   lexeme: String::from("=="),
                                   line_number,
                                   column_number,
                              });
                         } else {
                              tokens.push(Token {
                                   token_type: TokenTypes::Assign,
                                   lexeme: String::from("="),
                                   line_number,
                                   column_number,
                              });
                         }
                    },
     
                    '+' => {
                         if let Some(&'=') = chars.peek() {
                              chars.next();
                              tokens.push(Token {
                                   token_type: TokenTypes::PlusAssign,
                                   lexeme: String::from("+="),
                                   line_number,
                                   column_number,
                              });
                         } else {
                              tokens.push(Token {
                                   token_type: TokenTypes::Plus,
                                   lexeme: String::from("+"),
                                   line_number,
                                   column_number,
                              });
                         }
                    }
     
                    '-' => {
                         if let Some(&'=') = chars.peek() {
                              chars.next();
                              tokens.push(Token {
                                   token_type: TokenTypes::MinusEqual,
                                   lexeme: String::from("-="),
                                   line_number,
                                   column_number,
                              });
                         } else {
                              tokens.push(Token {
                                   token_type: TokenTypes::Minus,
                                   lexeme: String::from("-"),
                                   line_number,
                                   column_number,
                              });
                         }
                    }
     
                    '*' => {
                         if let Some(&'=') = chars.peek() {
                              chars.next();
                              tokens.push(Token {
                                   token_type: TokenTypes::AsteriskEqual,
                                   lexeme: String::from("*="),
                                   line_number,
                                   column_number,
                              });
                         } else {
                              tokens.push(Token {
                                   token_type: TokenTypes::Asterisk,
                                   lexeme: String::from("*"),
                                   line_number,
                                   column_number,
                              });
                         }
                    }
     
                    '/' => {
                         if let Some(&'=') = chars.peek() {
                              chars.next();
                              tokens.push(Token {
                                   token_type: TokenTypes::DivideEqual,
                                   lexeme: String::from("/="),
                                   line_number,
                                   column_number,
                              });
                         } else {
                              tokens.push(Token {
                                   token_type: TokenTypes::Divide,
                                   lexeme: String::from("/"),
                                   line_number,
                                   column_number,
                              });
                         }
                    }
     
                    '%' => {
                         if let Some(&'=') = chars.peek() {
                              chars.next();
                              tokens.push(Token {
                                   token_type: TokenTypes::ModuloEqual,
                                   lexeme: String::from("%="),
                                   line_number,
                                   column_number,
                              });
                         } else {
                              tokens.push(Token {
                                   token_type: TokenTypes::Modulo,
                                   lexeme: String::from("%"),
                                   line_number,
                                   column_number,
                              });
                         }
                    }
     
                    '<' => {
                         if let Some(&'=') = chars.peek() {
                              chars.next();
                              tokens.push(Token {
                                   token_type: TokenTypes::LessEqual,
                                   lexeme: String::from("<="),
                                   line_number,
                                   column_number,
                              });
                         } 
     
                         if let Some(&'>') = chars.peek() {
                              chars.next();
                              tokens.push(Token {
                                   token_type: TokenTypes::NotEqual,
                                   lexeme: String::from("<>"),
                                   line_number,
                                   column_number,
                              });
                         } else {
                              tokens.push(Token {
                                   token_type: TokenTypes::Less,
                                   lexeme: String::from("<"),
                                   line_number,
                                   column_number,
                              });
                         }
                    }
     
                    '>' => {
                         if let Some(&'=') = chars.peek() {
                              chars.next();
                              tokens.push(Token {
                                   token_type: TokenTypes::GreaterEqual,
                                   lexeme: String::from(">="),
                                   line_number,
                                   column_number,
                              });
                         } else {
                              tokens.push(Token {
                                   token_type: TokenTypes::Greater,
                                   lexeme: String::from(">"),
                                   line_number,
                                   column_number,
                              });
                         }
                    }
     
                    '!' => {
                         if let Some(&'=') = chars.peek() {
                              chars.next();
                              tokens.push(Token {
                                   token_type: TokenTypes::NotEqual,
                                   lexeme: String::from("!="),
                                   line_number,
                                   column_number,
                              });
                         } else {
                              tokens.push(Token {
                                   token_type: TokenTypes::Not,
                                   lexeme: String::from("!"),
                                   line_number,
                                   column_number,
                              });
                         }
                    }
     
                    '.' => tokens.push(Token {
                         token_type: TokenTypes::Eof,
                         lexeme: String::from("."),
                         line_number,
                         column_number,
                    }),
     
                    _ => {
                         tokens.push(Token {
                             token_type: TokenTypes::Error,
                             lexeme: String::from("".to_owned() + &c.to_string()),
                             line_number,
                             column_number,
                        });
                   }
               }
          }
     
          valid_table.set_header(vec!["Token Type", "Line Number", "Column Number", "Lexeme"]);
          error_table.set_header(vec!["Token Type", "Line Number", "Column Number", "Lexeme"]);
          
          for token in &tokens {
               if token.token_type == TokenTypes::Error {
                    error_table.add_row(vec![
                         token.token_type.to_string(),
                         token.lexeme.to_string(),
                         token.line_number.to_string(),
                         token.column_number.to_string(),
                    ]);
               }

               else {
                    valid_table.add_row(vec![
                         token.token_type.to_string(),
                         token.lexeme.to_string(),
                         token.line_number.to_string(),
                         token.column_number.to_string(),
                    ]);
               }
          }

          // let tokens = get_next_token(&buffer1);
          // println!("{:?}", tokens.unwrap()); 
          writeln!(valid_file, "{}", valid_table).expect("Unable to write to file");
          writeln!(error_file, "{}", error_table).expect("Unable to write to file");

          Ok(tokens)
     }   
}