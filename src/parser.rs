use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum NonTerminal {
     Program,
     FuncDecl,
     FuncDef,
     FuncDefRight,
     Params,
     ParamsRight,
     FuncName,
     Declarations,
     Decl,
     DeclRight,
     Type,
     Varlist,
     VarlistRight,
     StatementSequence,
     Statement,
     StatementSequenceRight,
     OptionElse,
     Expr,
     Term,
     TermRight,
     VarRight,
     Var,
     Comp,
     BranchFactorParen,
     BranchFactor,
     BranchFactorRight,
     BranchTerm,
     BranchTermRight,
     BranchExpression,
     ExpressionSequenceRight,
     ExpressionSequence,
     Factor,
     FactorRight,
     FactorParen,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Terminal {
     Def,
     Type,
     Ident,
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
     IntegerLiteral,
     DoubleLiteral,
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

impl Terminal {
     fn matches_token(&self, token: &Token) -> bool {
          match (self, &token.token_type) {
               (Terminal::Def, TokenTypes::Def) => true,
               (Terminal::Type, TokenTypes::Type(_)) => true,
               (Terminal::Ident, TokenTypes::Ident(_)) => true,
               (Terminal::LParen, TokenTypes::LParen) => true,
               (Terminal::RParen, TokenTypes::RParen) => true,
               (Terminal::Comma, TokenTypes::Comma) => true,
               (Terminal::Semicolon, TokenTypes::Semicolon) => true,
               (Terminal::Assign, TokenTypes::Assign) => true,
               (Terminal::Plus, TokenTypes::Plus) => true,
               (Terminal::PlusAssign, TokenTypes::PlusAssign) => true,
               (Terminal::Minus, TokenTypes::Minus) => true,
               (Terminal::MinusEqual, TokenTypes::MinusEqual) => true,
               (Terminal::Asterisk, TokenTypes::Asterisk) => true,
               (Terminal::AsteriskEqual, TokenTypes::AsteriskEqual) => true,
               (Terminal::Divide, TokenTypes::Divide) => true,
               (Terminal::DivideEqual, TokenTypes::DivideEqual) => true,
               (Terminal::Modulo, TokenTypes::Modulo) => true,
               (Terminal::ModuloEqual, TokenTypes::ModuloEqual) => true,
               (Terminal::If, TokenTypes::If) => true,
               (Terminal::Then, TokenTypes::Then) => true,
               (Terminal::Else, TokenTypes::Else) => true,
               (Terminal::Fi, TokenTypes::Fi) => true,
               (Terminal::While, TokenTypes::While) => true,
               (Terminal::Do, TokenTypes::Do) => true,
               (Terminal::Od, TokenTypes::Od) => true,
               (Terminal::Print, TokenTypes::Print) => true,
               (Terminal::Return, TokenTypes::Return) => true,
               (Terminal::Eof, TokenTypes::Eof) => true,
               (Terminal::IntegerLiteral, TokenTypes::IntegerLiteral(_)) => true,
               (Terminal::DoubleLiteral, TokenTypes::DoubleLiteral(_)) => true,
               (Terminal::Or, TokenTypes::Or) => true,
               (Terminal::And, TokenTypes::And) => true,
               (Terminal::Not, TokenTypes::Not) => true,
               (Terminal::Less, TokenTypes::Less) => true,
               (Terminal::Greater, TokenTypes::Greater) => true,
               (Terminal::Equal, TokenTypes::Equal) => true,
               (Terminal::LessEqual, TokenTypes::LessEqual) => true,
               (Terminal::GreaterEqual, TokenTypes::GreaterEqual) => true,
               (Terminal::NotEqual, TokenTypes::NotEqual) => true,
               (Terminal::LBracket, TokenTypes::LBracket) => true,
               (Terminal::RBracket, TokenTypes::RBracket) => true,
               (Terminal::Error, TokenTypes::Error) => true,
               _ => false,
          }
     }
}

#[derive(Debug)]
enum Symbol {
     NonTerminal(NonTerminal),
     Terminal(Terminal),
}

#[derive(Debug)]
struct ParseTree {
     symbol: Symbol,
     children: Vec<ParseTree>,
}

impl ParseTree {
     fn new(symbol: Symbol) -> Self {
          ParseTree {
               symbol,
               children: Vec::new(),
          }
     }
 
     fn add_child(&mut self, child: ParseTree) {
          self.children.push(child);
     }
}

struct Parser {
     first: HashMap<NonTerminal, Vec<Terminal>>,
     follow: HashMap<NonTerminal, Vec<Terminal>>,
}

impl Parser {
     fn initialize_first(&mut self) {
          self.first.insert("program", vec!["def", "int", "double", "if", "while", "print", "return", "ID"]);
          self.first.insert("funcDecl", vec!["def", "EPSILON"]);
          self.first.insert("funcDef", vec!["def"]);
          self.first.insert("funcDefRight", vec!["def", "EPSILON"]);
          self.first.insert("params", vec!["int", "double", "EPSILON"]);
          self.first.insert("paramsRight", vec![",", "EPSILON"]);
          self.first.insert("funcName", vec!["ID"]);
          self.first.insert("declarations", vec!["int", "double", "EPSILON"]);
          self.first.insert("decl", vec!["int", "double"]);
          self.first.insert("declRight", vec!["int", "double", "EPSILON"]);
          self.first.insert("type", vec!["int", "double"]);
          self.first.insert("varlist", vec!["ID"]);
          self.first.insert("varlistRight", vec![",", "EPSILON"]);
          self.first.insert("statementSequence", vec!["if", "while", "print", "return", "ID", "EPSILON"]);
          self.first.insert("statement", vec!["if", "while", "print", "return", "ID", "EPSILON"]);
          self.first.insert("statementSequenceRight", vec![";", "EPSILON"]);
          self.first.insert("optionElse", vec!["else", "EPSILON"]);
          self.first.insert("expr", vec!["ID", "NUMBER", "("]);
          self.first.insert("term", vec!["ID", "NUMBER", "("]);
          self.first.insert("termRight", vec!["+", "-", "EPSILON"]);
          self.first.insert("varRight", vec!["[", "EPSILON"]);
          self.first.insert("var", vec!["ID"]);
          self.first.insert("comp", vec!["<", ">", "==", "<=", ">=", "<>"]);
          self.first.insert("branchFactorParen", vec!["(", "not", "ID", "NUMBER", "EPSILON"]);
          self.first.insert("branchFactor", vec!["(", "not"]);
          self.first.insert("branchFactorRight", vec!["and", "EPSILON"]);
          self.first.insert("branchTerm", vec!["(", "not"]);
          self.first.insert("branchTermRight", vec!["or", "EPSILON"]);
          self.first.insert("branchExpression", vec!["(", "not"]);
          self.first.insert("expressionSequenceRight", vec![",", "EPSILON"]);
          self.first.insert("expressionSequence", vec!["(", "ID", "NUMBER"]);
          self.first.insert("factor", vec!["(", "ID", "NUMBER"]);
          self.first.insert("factorRight", vec!["*", "/", "%", "EPSILON"]);
          self.first.insert("factorParen", vec!["(", "EPSILON"]);
     }

    fn initialize_follow(&mut self) {
          self.follow.insert("program", vec!["$"]);
          self.follow.insert("funcDecl", vec!["int", "double", "if", "while", "print", "return", "ID"]);
          self.follow.insert("funcDef", vec![";"]);
          self.follow.insert("funcDefRight", vec![";"]);
          self.follow.insert("params", vec![")"]);
          self.follow.insert("paramsRight", vec![")"]);
          self.follow.insert("funcName", vec!["("]);
          self.follow.insert("declarations", vec!["if", "while", "print", "return", "ID"]);
          self.follow.insert("decl", vec![";"]);
          self.follow.insert("declRight", vec![";"]);
          self.follow.insert("type", vec!["ID"]);
          self.follow.insert("varlist", vec![";", ",", ".", "(", ")", "]", "[", "then", "+", "-", "", "/", "%", "==", "<>", "<", ">"]);
          self.follow.insert("varlistRight", vec![";", ",", ".", "(", ")", "]", "[", "then", "+", "-", "", "/", "%", "==", "<>", "<", ">"]);
          self.follow.insert("statementSequence", vec![".", "fed", "fi", "od", "else"]);
          self.follow.insert("statement", vec![".", ";", "fed", "fi", "od", "else"]);
          self.follow.insert("statementSequenceRight", vec![".", ";", "fed", "fi", "od", "else"]);
          self.follow.insert("optionElse", vec![".", ";", "fed", "fi", "od", "else"]);
          self.follow.insert("expr", vec![".", ";", "fed", "fi", "od", "else", ")", "=", ">", "<", "]"]);
          self.follow.insert("term", vec![".", ";", "fed", "fi", "od", "else", ")", "=", ">", "<", "]", "+", "-", "", "/"]);
          self.follow.insert("termRight", vec![".", ";", "fed", "fi", "od", "else", ")", "=", ">", "<", "]", "+", "-", "", "/"]);
          self.follow.insert("varRight", vec![";", ",", ".", "(", ")", "]", "[", "then", "+", "-", "", "/", "%", "==", "<>", "<", ">"]);
          self.follow.insert("var", vec![";", ",", ".", "(", ")", "]", "[", "then", "+", "-", "", "/", "%", "==", "<>", "<", ">"]);
          self.follow.insert("comp", vec![""]);
          self.follow.insert("branchFactorParen", vec!["then", "do", ")", "or", "and"]);
          self.follow.insert("branchFactor", vec!["then", "do", ")", "or", "and"]);
          self.follow.insert("branchFactorRight", vec!["then", "do", ")", "or", "and"]);
          self.follow.insert("branchTerm", vec!["then", "do", ")", "or", "and"]);
          self.follow.insert("branchTermRight", vec!["then", "do", ")", "or", "and"]);
          self.follow.insert("branchExpression", vec!["then", "do", ")", "or"]);
          self.follow.insert("expressionSequenceRight", vec![")"]);
          self.follow.insert("expressionSequence", vec![")"]);
          self.follow.insert("factor", vec![".", ";", "fed", "fi", "od", "else", ")", "=", ">", "<", "]", "+", "-", "", "/"]);
          self.follow.insert("factorRight", vec![".", ";", "fed", "fi", "od", "else", ")", "=", ">", "<", "]", "+", "-", "", "/"]);
          self.follow.insert("factorParen", vec![".",";","fed","fi","od","else",")","=",">","<","]","+","-","*","/"]);
     }