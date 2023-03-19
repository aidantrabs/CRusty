/*
     @Description: Enum of all possible tokens
     @Params: None
     @Returns: None
*/
#[derive(Debug)]
pub enum Token {
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
     MinusAssign,
     Asterisk,
     AsteriskAssign,
     Slash,
     SlashAssign,
     Percent,
     PercentAssign,
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
}

/*
     @Description: Lexical analyzer
     @Params: input - String to be tokenized
     @Returns: Result<Vec<Token>, String> - Vector of tokens or error message
*/
pub fn lex(input: &str) -> Result<Vec<Token>, String> {
     let mut tokens = Vec::new();
     let mut chars = input.chars().peekable();
     let mut line_number = 1;

     while let Some(c) = chars.next() {
          match c {
               '\n' => line_number += 1,
               ' ' | '\t' | '\r' => continue,
               'a'..='z' | 'A'..='Z' => {
                    let mut ident = c.to_string();
                    while let Some(&next_char) = chars.peek() {
                         match next_char {
                              'a'..='z' | 'A'..='Z' | '0'..='9' => {
                                   ident.push(next_char);
                                   chars.next();
                              }
                              _ => break,
                         }
                    }

                    match ident.as_str() {
                         "def" => tokens.push(Token::Def),
                         "int" => tokens.push(Token::Type("int".to_owned())),
                         "double" => tokens.push(Token::Type("double".to_owned())),
                         "if" => tokens.push(Token::If),
                         "then" => tokens.push(Token::Then),
                         "else" => tokens.push(Token::Else),
                         "fi" => tokens.push(Token::Fi),
                         "while" => tokens.push(Token::While),
                         "do" => tokens.push(Token::Do),
                         "od" => tokens.push(Token::Od),
                         "print" => tokens.push(Token::Print),
                         "return" => tokens.push(Token::Return),
                         _ => tokens.push(Token::Ident(ident)),
                    }
               }

               '0'..='9' => {
                    let mut num_str = c.to_string();
                    let mut is_float = false;
                    while let Some(&next_char) = chars.peek() {
                         match next_char {
                              '0'..='9' => {
                                   num_str.push(next_char);
                                   chars.next();
                              }
                              '.' if !is_float => {
                                   num_str.push(next_char);
                                   chars.next();
                                   is_float = true;
                              }

                              'a'..='z' | 'A'..='Z' => {
                                   return Err(format!("Invalid token: '{}' at line {}", next_char, line_number));
                              }
                         
                              _ => break,
                         }
                    }

                    if is_float {
                         if let Ok(num) = num_str.parse() {
                              tokens.push(Token::DoubleLiteral(num));
                         }
                    } else if let Ok(num) = num_str.parse() {
                         tokens.push(Token::IntegerLiteral(num));
                    }
               }

               '(' => tokens.push(Token::LParen),
               ')' => tokens.push(Token::RParen),
               ',' => tokens.push(Token::Comma),
               ';' => tokens.push(Token::Semicolon),
               '=' => match chars.peek() {
                    Some(&'=') => {
                         chars.next();
                         tokens.push(Token::Equal);
                    }
                    _ => tokens.push(Token::Assign),
               },
               '+' => match chars.peek() {
                    Some(&'=') => {
                         chars.next();
                         tokens.push(Token::PlusAssign);
                    }
                    _ => tokens.push(Token::Plus),
               },
               '-' => match chars.peek() {
                    Some(&'=') => {
                         chars.next();
                         tokens.push(Token::MinusAssign);
                    }
                    _ => tokens.push(Token::Minus),
               },
               '*' => match chars.peek() {
                    Some(&'=') => {
                         chars.next();
                         tokens.push(Token::AsteriskAssign);
                    }
                    _ => tokens.push(Token::Asterisk),
               },
               '/' => match chars.peek() {
                    Some(&'=') => {
                         chars.next();
                         tokens.push(Token::SlashAssign);
                    }
                    _ => tokens.push(Token::Slash),
               },
               '%' => match chars.peek() {
                    Some(&'=') => {
                         chars.next();
                         tokens.push(Token::PercentAssign);
                    }
                    _ => tokens.push(Token::Percent),
               },
               '!' => match chars.peek() {
                    Some(&'=') => {
                         chars.next();
                         tokens.push(Token::NotEqual);
                    }
                    _ => tokens.push(Token::Not),
               },
               '<' => match chars.peek() {
                    Some(&'=') => {
                         chars.next();
                         tokens.push(Token::LessEqual);
                    }
                    _ => tokens.push(Token::Less),
               },
               '>' => match chars.peek() {
                    Some(&'=') => {
                         chars.next();
                         tokens.push(Token::GreaterEqual);
                    }
                    _ => tokens.push(Token::Greater),
               },
               '|' => match chars.peek() {
                    Some(&'|') => {
                         chars.next();
                         tokens.push(Token::Or);
                    }
                    _ => return Err(format!("Invalid token: '{}' at line {}", c, line_number)),
               },
               '&' => match chars.peek() {
                    Some(&'&') => {
                         chars.next();
                         tokens.push(Token::And);
                    }
                    _ => return Err(format!("Invalid token: '{}' at line {}", c, line_number)),
               },
               '[' => tokens.push(Token::LBracket),
               ']' => tokens.push(Token::RBracket),
               _ => return Err(format!("Invalid token: '{}' at line {}", c, line_number)),
          }
     }
     Ok(tokens)
}