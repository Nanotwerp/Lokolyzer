# Lokolyzer
An attempt at making a lexical + syntax analyzer in Rust

In its final form, this analyzer will be able to do the operations of addition, subtraction, multiplication, division, and modulus; this is in addition to being able to assign and compare variables. 

Grammar:
<lokolyzer> -> '$'<stmt_list>'?'
<stmt_list  -> <stmt> ';' {<stmt>';'}

stmt -> <block>
<int_declaration> -> "rst"
<short_declaration -> "rort"
<long declaration> -> "rong"
<declare> -> <int_declaration> | <short_declaration> | <long_declaration> 'id' ';'

<block> -> '{' { <stmt> ;} '}'
<assignment> -> 'id' '=' <expr ';'
<expr> -> <term> {('/'|'*'|'%') <term> }
<term> -> <factor> {('+'|'-') <factor> }
factor -> 'id' | 'int_lit' | '('<expr>')'

<ineq> -> <bex> {('<'|'>'|'~'|'^') <bex> }
<bterm> -> <bfactor> {('+'|'-') <bfactor> }
<bfactor -> 'id' | 'int_lit' | '('<bex>')'
