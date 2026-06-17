# Anix : Another interpreter experiment

Implementation of [Crafting Interpreter](https://craftinginterpreters.com/) in Rust.

## Grammar

`expression = assignment ;`  
`assignment = IDENTIFIER "=" equality | equality`
`equality = comparison ( ( "!=" | "==" ) comparison )* ;`  
`comparison = term ( ( ">" | ">=" | "<" | "<=" ) term )* ;`  
`term = factor ( ( "-" | "+" ) factor )* ;`  
`factor = unary ( ( "/" | "*" ) unary )* ;`  
`unary = ( "!" | "-" ) unary | primary ;`  
`primary = NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;`  

`program = declaration* EOF ;`  
`declaration = varDecl | statement ;`  
`statement = exprStmt | printStmt ;`  
`exprStmt = expression ";" ;`  
`printStmt = "print" expression ";" ;`  

`varDecl = "var" IDENTIFIER ( "=" expression )? ";" ;`  
