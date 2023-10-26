use std::collections::{HashMap, HashSet};

/*
     @Description: Static variables for the first set, follow set, terminals, nonterminals, grammar, first and follow, and lookahead
     @Params: None
     @Returns: None
*/
static mut FIRST_SET_DICT: HashMap<String, HashSet<String>> = HashMap::new();
static mut FOLLOW_SET_DICT: HashMap<String, HashSet<String>> = HashMap::new();
static mut TERMINALS: HashSet<String> = HashSet::new();
static mut NONTERMINALS: HashSet<String> = HashSet::new();
static mut GRAMMAR: Vec<String> = Vec::new();
static mut FIRST_AND_FOLLOW: Vec<(String, String, String)> = Vec::new();
static mut LOOKAHEAD: HashMap<String, HashMap<String, String>> = HashMap::new();
static mut START_SYMBOL: String = String::new();

/*
     @Description: Calculates the first set, follow set, and lookahead for each nonterminal
     @Params: None
     @Returns: None
*/
fn sets_and_table() {
     unsafe {
          for rule in &GRAMMAR {
               let mut iter = rule.split(" ::= ");
               let lhs = iter.next().unwrap();
               NONTERMINALS.insert(lhs.to_string());
          }

          for rule in &GRAMMAR {
               let mut iter = rule.split(" ::= ");
               let lhs = iter.next().unwrap();
               let rhs_symbols = iter.next().unwrap().split(" ");
               for symbol in rhs_symbols {
                    if !NONTERMINALS.contains(symbol) {
                         TERMINALS.insert(symbol.to_string());
                    }
               }
          }

          first_set();
          follow_set();

          for rule in &FIRST_AND_FOLLOW {
               let first_symbols = rule.1.split(" | ");
               let follow_symbols = rule.2.split(" | ");
               FIRST_SET_DICT.insert(rule.0.to_string(), HashSet::new());
               FOLLOW_SET_DICT.insert(rule.0.to_string(), HashSet::new());
               for symbol in first_symbols {
                    FIRST_SET_DICT.get_mut(&rule.0).unwrap().insert(symbol.to_string());
               }
               for symbol in follow_symbols {
                    FOLLOW_SET_DICT.get_mut(&rule.0).unwrap().insert(symbol.to_string());
               }
          }

          lookahead_one();
     }
}

/*
     @Description: Calculates the first set for each nonterminal
     @Params: None
     @Returns: None
*/
fn first_set() {
     unsafe {
          for rule in &GRAMMAR {
               let mut iter = rule.split(" ::= ");
               let lhs = iter.next().unwrap();
               let rhs_symbols = iter.next().unwrap().split(" ");

               NONTERMINALS.insert(lhs.to_string());

               for symbol in rhs_symbols {
                    let first_s = FIRST_SET_DICT.entry(symbol.to_string()).or_insert(HashSet::new());

                    if symbol == "𝛜" {
                         first_s.insert(symbol.to_string());
                         break;
                    } else {
                         let first_rhs = FIRST_SET_DICT.get(symbol).unwrap();
                         if !first_rhs.contains("𝛜") {
                              first_s.extend(first_rhs.iter().cloned());
                         break;
                         } else {
                              first_s.extend(first_rhs.iter().cloned().filter(|s| *s != "𝛜"));
                              first_s.extend(FIRST_SET_DICT.get(lhs).unwrap().iter().cloned());
                         }
                    }
               }

               if !FIRST_SET_DICT.contains_key(lhs) {
                    FIRST_SET_DICT.insert(lhs.to_string(), HashSet::new());
               }
               FIRST_SET_DICT.get_mut(lhs).unwrap().insert("𝛜".to_string());
          }
     }
}

/*
     @Description: Calculates the follow set for each nonterminal
     @Params: None
     @Returns: None
*/
fn follow_set() {
     unsafe {
          START_SYMBOL = GRAMMAR[0].split(" ::= ").next().unwrap().to_string();
          FOLLOW_SET_DICT.insert(START_SYMBOL.to_string(), HashSet::new());
          FOLLOW_SET_DICT.get_mut(&START_SYMBOL).unwrap().insert("$".to_string());

          for rule in &GRAMMAR {
               let mut iter = rule.split(" ::= ");
               let lhs = iter.next().unwrap();
               let rhs_symbols = iter.next().unwrap().split(" ");

               for (index, symbol) in rhs_symbols.enumerate() {
                    if NONTERMINALS.contains(symbol) {
                         let follow_s = FOLLOW_SET_DICT.entry(symbol.to_string()).or_insert(HashSet::new());
                         let mut follow_rhs = FOLLOW_SET_DICT.get(lhs).unwrap().iter().cloned();
                         if index < rhs_symbols.clone().count() - 1 {
                              let mut first_rhs = FIRST_SET_DICT.get(rhs_symbols.clone().nth(index + 1).unwrap()).unwrap().iter().cloned();
                              if first_rhs.contains("𝛜") {
                                   first_rhs = first_rhs.filter(|s| *s != "𝛜");
                                   follow_rhs.extend(first_rhs);
                                   follow_rhs.extend(FOLLOW_SET_DICT.get(lhs).unwrap().iter().cloned());
                              } else {
                                   follow_rhs.extend(first_rhs);
                              }
                         } else {
                              follow_rhs.extend(FOLLOW_SET_DICT.get(lhs).unwrap().iter().cloned());
                         }
                         follow_s.extend(follow_rhs);
                    }
               }
          }
     }
}

/*
     @Description: Calculates the lookahead for each nonterminal
     @Params: None
     @Returns: None
*/
fn lookahead_one() {
     unsafe {
          for rule in &GRAMMAR {
               let mut iter = rule.split(" ::= ");
               let lhs = iter.next().unwrap();
               let rhs_symbols = iter.next().unwrap().split(" ");

               for symbol in rhs_symbols {
                    if NONTERMINALS.contains(symbol) {
                         LOOKAHEAD.insert(symbol.to_string(), HashMap::new());
                    }
               }
          }

          for rule in &GRAMMAR {
               let mut iter = rule.split(" ::= ");
               let lhs = iter.next().unwrap();
               let rhs_symbols = iter.next().unwrap().split(" ");

               for (index, symbol) in rhs_symbols.enumerate() {
                    if NONTERMINALS.contains(symbol) {
                         let mut lookahead_s = LOOKAHEAD.get_mut(symbol).unwrap();
                         let mut first_rhs = FIRST_SET_DICT.get(rhs_symbols.clone().nth(index + 1).unwrap()).unwrap().iter().cloned();
                         if first_rhs.contains("𝛜") {
                              first_rhs = first_rhs.filter(|s| *s != "𝛜");
                              lookahead_s.extend(first_rhs);
                              lookahead_s.extend(FOLLOW_SET_DICT.get(lhs).unwrap().iter().cloned());
                         } else {
                              lookahead_s.extend(first_rhs);
                         }
                    }
               }
          }
     }
}

/*
     @Description: Calculates the parsing table
     @Params: None
     @Returns: None
*/
fn parse_table() {
     unsafe {
          for rule in &GRAMMAR {
               let mut iter = rule.split(" ::= ");
               let lhs = iter.next().unwrap();
               let rhs_symbols = iter.next().unwrap().split(" ");

               for symbol in rhs_symbols {
                    if NONTERMINALS.contains(symbol) {
                         for lookahead in LOOKAHEAD.get(symbol).unwrap().keys() {
                              TABLE.insert((lhs.to_string(), lookahead.to_string()), rule.to_string());
                         }
                    } else {
                         TABLE.insert((lhs.to_string(), symbol.to_string()), rule.to_string());
                    }
               }
          }
     }
}

/*
     @Description: Parses the input string
     @Params: input - the input string to parse
     @Returns: None
*/
pub fn parse(input: &str) -> Result<(), String> {
     unsafe {
          let mut stack = vec!["$".to_string(), START_SYMBOL.to_string()];
          let mut input_iter = input.split(" ");
          let mut input_symbol = input_iter.next().unwrap().to_string();
          let mut stack_symbol = stack.pop().unwrap();

          while stack_symbol != "$" {
               if stack_symbol == input_symbol {
                    input_symbol = input_iter.next().unwrap().to_string();
                    stack_symbol = stack.pop().unwrap();
               } else if TERMINALS.contains(&stack_symbol) {
                    return Err(format!("Error: Expected {}, found {}", stack_symbol, input_symbol));
               } else if TABLE.contains_key(&(stack_symbol.to_string(), input_symbol.to_string())) {
                    let rule = TABLE.get(&(stack_symbol.to_string(), input_symbol.to_string())).unwrap();
                    let mut iter = rule.split(" ::= ");
                    let lhs = iter.next().unwrap();
                    let rhs_symbols = iter.next().unwrap().split(" ").rev();
                    for symbol in rhs_symbols {
                         stack.push(symbol.to_string());
                    }
               } else {
                    return Err(format!("Error: No rule for {} and {}", stack_symbol, input_symbol));
               }
               stack_symbol = stack.pop().unwrap();
          }
          Ok(())
     }
}

/*
     @Description: Prints the grammar
     @Params: None
     @Returns: None
*/
pub fn print_grammar() {
     unsafe {
          println!("Grammar:");
          for rule in &GRAMMAR {
               println!("\t{}", rule);
          }
     }
}

/*
     @Description: Prints the terminals
     @Params: None
     @Returns: None
*/
pub fn print_terminals() {
     unsafe {
          println!("Terminals:");
          for terminal in &TERMINALS {
               println!("\t{}", terminal);
          }
     }
}

/*
     @Description: Prints the nonterminals
     @Params: None
     @Returns: None
*/
pub fn print_nonterminals() {
     unsafe {
          println!("Nonterminals:");
          for nonterminal in &NONTERMINALS {
               println!("\t{}", nonterminal);
          }
     }
}

/*
     @Description: Prints the first set
     @Params: None
     @Returns: None
*/
pub fn print_first_set() {
     unsafe {
          println!("First Set:");
          for (symbol, first_set) in &FIRST_SET_DICT {
               print!("\t{}: ", symbol);
               for first in first_set {
                    print!("{} ", first);
               }
               println!("");
          }
     }
}

/*
     @Description: Prints the follow set
     @Params: None
     @Returns: None
*/
pub fn print_follow_set() {
     unsafe {
          println!("Follow Set:");
          for (symbol, follow_set) in &FOLLOW_SET_DICT {
               print!("\t{}: ", symbol);
               for follow in follow_set {
                    print!("{} ", follow);
               }
               println!("");
          }
     }
}

/*
     @Description: Prints the lookahead
     @Params: None
     @Returns: None
*/
pub fn print_lookahead() {
     unsafe {
          println!("Lookahead:");
          for (symbol, lookahead) in &LOOKAHEAD {
               print!("\t{}: ", symbol);
               for lookahead in lookahead {
                    print!("{} ", lookahead.0);
               }
               println!("");
          }
     }
}

/*
     @Description: Prints the parsing table
     @Params: None
     @Returns: None
*/
pub fn print_table() {
     unsafe {
          println!("Parsing Table:");
          for ((lhs, lookahead), rule) in &TABLE {
               println!("\t{} {} {}", lhs, lookahead, rule);
          }
     }
}
