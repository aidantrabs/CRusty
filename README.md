<div align="center">
  
```
  ______  _______                      __              
 /      \|       \                    |  \             
|  ▓▓▓▓▓▓\ ▓▓▓▓▓▓▓\__    __  _______ _| ▓▓_   __    __ 
| ▓▓   \▓▓ ▓▓__| ▓▓  \  |  \/       \   ▓▓ \ |  \  |  \
| ▓▓     | ▓▓    ▓▓ ▓▓  | ▓▓  ▓▓▓▓▓▓▓\▓▓▓▓▓▓ | ▓▓  | ▓▓
| ▓▓   __| ▓▓▓▓▓▓▓\ ▓▓  | ▓▓\▓▓    \  | ▓▓ __| ▓▓  | ▓▓
| ▓▓__/  \ ▓▓  | ▓▓ ▓▓__/ ▓▓_\▓▓▓▓▓▓\ | ▓▓|  \ ▓▓__/ ▓▓
 \▓▓    ▓▓ ▓▓  | ▓▓\▓▓    ▓▓       ▓▓  \▓▓  ▓▓\▓▓    ▓▓
  \▓▓▓▓▓▓ \▓▓   \▓▓ \▓▓▓▓▓▓ \▓▓▓▓▓▓▓    \▓▓▓▓ _\▓▓▓▓▓▓▓
                                             |  \__| ▓▓
                                              \▓▓    ▓▓
                                               \▓▓▓▓▓▓ 
```

### A lightweight lexer and parser for the EZ lang

![GitHub](https://img.shields.io/github/license/aidantrabs/CRusty?style=flat-square)
  
</div>

## Description :pushpin: 
- Takes a `.ez` file as input and utilizes a double-buffer method to lexically analyze the language.
- Utilizes FIRST and FOLLOW sets to then parse the production rules.

#### EZ Lang Grammar
> The rules for EZ lang
```
<program> ::= <fdecls> <declarations> <statement_seq>.
<fdecls> ::= <fdec>; | <fdecls> <fdec>; |
<fdec> ::= def <type> <fname> ( <params> ) <declarations> <statement_seq> fed
<params> ::= <type> <var> | <type> <var> , <params> |
<fname> ::= <id>
<declarations> ::= <decl>; | <declarations> <decl>; |
<decl> := <type> <varlist>
<type> := int | double
<varlist> ::= <var>, <varlist> | <var>
<statement_seq> ::= <statement> | <statement>; <statement_seq>
<statement> ::= <var> = <expr> |
if <bexpr> then <statement_seq> fi |
if <bexpr> then <statement_seq> else <statement_seq> fi |
while <bexpr> do <statement_seq> od |
print <expr> |
return <expr> |
<expr> ::= <expr> + <term> | <expr> - <term> | <term>
<term> ::= <term> * <factor> | <term> / <factor> | <term> % <factor> |
<factor>
<factor> ::= <var> | <number> | (<expr>) | <fname>(<exprseq>)
<exprseq> ::= <expr>, <exprseq> | <expr> |
<bexpr> ::= <bexpr> or <bterm> | <bterm>
<bterm> ::= <bterm> and <bfactor> | <bfactor>
<bfactor> ::= (<bexpr>) | not <bfactor> | (<expr> <comp> <expr>)
<comp> ::= < | > | == | <= | >= | <>
<var> ::= <id> | <id>[<expr>]
<letter> ::= a | b | c | ... | z
<digit> ::= 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 0
<id> ::= <letter> | <id><letter> | <id><digit>
<number> ::= <integer> | <double>...
```

#### Example: Greatest Common Divisor Algorithm

```cpp
def int gcd(int a, int b)
	if (a==b) then
		return (a) 
	fi;
	if (a>b) then
		return(gcd(a-b, b))
	else 
		return(gcd(a, b-a)) 
	fi;
fed;
print gcd(21,15).
```

## Usage :pencil:

### CLI

<br/>

> Enter source directory
```sh
$ cd src
```

> Run the program
```sh
$ cargo run <TestFile>.cp
```
