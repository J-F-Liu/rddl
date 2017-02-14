# RDDL

[![Build Status](https://travis-ci.org/J-F-Liu/rddl.png)](https://travis-ci.org/J-F-Liu/rddl)

Refined Data Description Language (RDDL) is based on [OpenDDL](http://openddl.org/), mainly replacing primitive data type names and literal syntax with the same in Rust.

## Benifits of RDDL

- concise text
- human readable and editable
- store arbitrary data
- strong typing

RDDL is suitable for describing database schemas, RPC protocols, 3D models, game objects and neural networks etc.

## Example
```
VertexArray
{
    f32
    {
        {1.0, 2.0, 3.0},
        {0.5, 0.0, 0.5},
        {0.0, -1.0, 4.0}
    }
}

Person $chuck
{
    Name {str "Charles"}
    Friends
    {
        ref {$alice, $bob}
    }
}
Person $alice { Name {str "Alice"} }
Person $bob { Name {str "Bob"} }
```

## Grammar

The grammar of RDDL can be expressed in PEG as:

```
data_file = data_item+

data_item = primitive_item | custom_item

primitive_item = primitive_type name? primitive_data

primitive_data = primitive_value | vector | matrix

vector = '{' (primitive_value (',' primitive_value)*)? '}'

matrix = '{' (vector (',' vector)*)? '}'

custom_item = identifier name? properties? '{' data_item* '}'

property = identifier '=' primitive_value

properties = '(' (property (',' property)*)? ')'

identifier = [A-za-z_][0-9A-za-z_]*

name = ('$' | '%') identifier

primitive_type = "bool" | "u8" | "u16" | "u32" | "u64" |
	"i8" | "i16" | "i32" | "i64" | "f32" | "f64" |
	 "str" | "ref" | "type"

primitive_value = bool | integer | float | string | reference | primitive_type

bool = "true" | "false"

integer = '-'? (hexical | octal | binary | decimal )

hexical = "0x" [0-9A-Fa-f]+ ('_' [0-9A-Fa-f]+)*

octal = "0o" [0-7]+ ('_' [0-7]+)*

binary = "0b" [01]+ ('_' [01]+)*

decimal = [0-9]+ ('_' [0-9]+)*

float = '-'? ([1-9][0-9]* | '0') ('.' [0-9]+)? ([eE] [+-]? [0-9]+)?

string = '"' ([^\\"] | escape_sequence | hex_char | unicode_char)* '"'

escape_sequence = '\\' ('\\' | '"' | '0' | 'n' | 'r' | 't')

hex_char = "\\x" [0-9A-Fa-f][0-9A-Fa-f]

unicode_char = "\\u{" [0-9A-Fa-f]{1,6} '}'

reference = name ('%' identifier)*

space = [ \t\r\n]*

```
`space` can be inserted between other tokens.
