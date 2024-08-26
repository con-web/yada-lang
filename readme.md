# yada-lang

### Yet Another Dumb Amateur Language

This is just a toy programming language for me to learn something about pl design and compiler theory/concepts.

#### Roadmap: 

1) choose token set 
2) basic lexer
3) reason about language semantics 
4) parser -> ast
5) reason about language semantics even more
6) refine token set / syntax
7) refine parser / ast
8) rinse and repeat from 3
9) if i make it until here: try to translate to llvm ir (or maybe mlir?)
10) build the actual fun stuff: interpreter, compiler, language-server, yada yada yada...

#### Current State: 

This pseudo source file with some example syntax: 
```yada-lang
struct MyStruct {
    field1: i32,
    field2: f64,
    
    fn new() -> Self {
        Self {0,0}
    }
}

fn my_func(param1: f64, _unused: bool) -> MyStruct {
    if param1 >= 35.0 {
        const a: i32 = 34;
        var b: f64 = 0;
        b += param1;
        return MyStruct{b, param1};
    } else {
        return MyStruct::new();
    }
}
```
... gets tokenized successfully by the lexer: 

```
STRUCT_KEYWORD: struct
IDENTIFIER: MyStruct
L_CURLY: {
IDENTIFIER: field1
COLON: :
IDENTIFIER: i32
COMMA: ,
IDENTIFIER: field2
COLON: :
IDENTIFIER: f64
COMMA: ,
FN_KEYWORD: fn
IDENTIFIER: new
L_PAREN: (
R_PAREN: )
RETURNS_OP: ->
IDENTIFIER: Self
L_CURLY: {
IDENTIFIER: Self
L_CURLY: {
INT_LITERAL: 0
COMMA: ,
INT_LITERAL: 0
R_CURLY: }
R_CURLY: }
R_CURLY: }
FN_KEYWORD: fn
IDENTIFIER: my_func
L_PAREN: (
IDENTIFIER: param1
COLON: :
IDENTIFIER: f64
COMMA: ,
IDENTIFIER: _unused
COLON: :
IDENTIFIER: bool
R_PAREN: )
RETURNS_OP: ->
IDENTIFIER: MyStruct
L_CURLY: {
IF_KEYWORD: if
IDENTIFIER: param1
GREATER_EQ: >=
FLOAT_LITERAL: 35.0
L_CURLY: {
CONST_KEYWORD: const
IDENTIFIER: a
COLON: :
IDENTIFIER: i32
ASSIGN: =
INT_LITERAL: 34
SEMICOLON: ;
VAR_KEYWORD: var
IDENTIFIER: b
COLON: :
IDENTIFIER: f64
ASSIGN: =
INT_LITERAL: 0
SEMICOLON: ;
IDENTIFIER: b
ADD_ASSIGN: +=
IDENTIFIER: param1
SEMICOLON: ;
RETURN_KEYWORD: return
IDENTIFIER: MyStruct
L_CURLY: {
IDENTIFIER: b
COMMA: ,
IDENTIFIER: param1
R_CURLY: }
SEMICOLON: ;
R_CURLY: }
ELSE_KEYWORD: else
L_CURLY: {
RETURN_KEYWORD: return
IDENTIFIER: MyStruct
COLON: :
COLON: :
IDENTIFIER: new
L_PAREN: (
R_PAREN: )
SEMICOLON: ;
R_CURLY: }
R_CURLY: }
EOF: 
```

I have also sketched out an idea for the AST data structure (stolen, ehm.. i mean borrowed from 
[here](https://github.com/rj45/gosie_c/)).

Next up is finalizing the ast and writing a pratt parser for it..

#### License

MIT