# Tydi-lang (2nd edition) syntax

## Literals

### Comment

Comments in Tydi is similar to the C-style line comments.

```cpp
//This is a comment in Tydi-lang
```

```cpp
/*this is a block
         comment*/
```

### Int 

Integer: a signed 128-bit integer.

```cpp
0 //normal integer 0
0x01234567894abcdef // hex integer
0x01234567894ABCDEF // hex integer is case insensitive
0o01234567 // octal integer
0b01010101 // binary integer
0b0000_0001 // you can use "_" as a separator anywhere except at the beginning of the integer.
```

### Float

Float: a signed 64-bit floating number.

```cpp
1.0
22.00205
```

### String

String must start and end with a ", and is allowed to have printable ascii characters and \\t, \\n, \\\\.

```cpp
"123" // this is a string
"this is \t another \n string"
```

### Boolean literal

```cpp
true
false // notice that "true" and "false" are case sensitive
```

### Identifier

Each character of an Identifier can be an English character (a-z, A-Z), a digit(0-9), or an underscore (_). Digit cannot be the first character.

Valid identifiers:
```cpp
abc
abc_
_abc
abc01564
```

Invalid identifier:
```cpp
0abc
```

Some specific identifiers are not allowed since they are used as keywords, they are: ```impl, streamlet, int, string, bool, float, instance, in, out```

## Basic types

Tydi-lang has 5 basic data types:

| keyword     | memory representation        | example                  |
|-------------|------------------------------|--------------------------|
| int         | signed 128-bit integer       | 0,1,10,-999              |
| float       | 64-bit floating point number | 0.0,1.0                  |
| string      | a collection of characters   | "1234"                   |
| bool        | boolean value                | true, false              |
| logic types | a logic type                 | Bit(8), Null, Group{...} |

And two extended types:

| extended types | memory representation             | example                            |
|----------------|-----------------------------------|------------------------------------|
| array          | a collection of any basic types   | [0,1,2,3,4] [0, "123", true, Null] |
| clockdomain    | a slash + a string(or identifier) | /"clock0" /clock_variable          |

Notice that the array itself doesn't have a specific type, its element has a definite basic type.

## Alias

Alias means binding an expression / a logical type / a value / a template instance to an identifier. For example:

```cpp
a = 1 + 2;   //a is an identifier and 1 + 2 is an expression
bit_8 = Bit(8);  //bind bit_8 to logic type "Bit(8)"
```

You can also specify the type of an alias:

```cpp
a : int = 1 + 2;
```

The ```: int``` is also called type indicator. For most cases, type indicator is optional and can be one of the basic data types. Notice that the type indicator cannot be a logic types because syntax like ```t: LogicTypes ``` means declaring a logic type.

## Expression, operator and Term
An expression is a combination of terms and operators. For example: expression `1+2` has 2 terms(`1` and `2`) and 1 operator(`+`). Terms include values of basic types and variables. The precedence of Tydi-lang operators basicly follows the C++ operator precedence:
| **Precedence** | **Operator** |  **Description**  | **Associativity** |                            **Allowed types**                            |                                  **Result type**                                  |    **Example**   |
|:--------------:|:------------:|:-----------------:|:-----------------:|:-----------------------------------------------------------------------:|:---------------------------------------------------------------------------------:|:----------------:|
|        0       |       -      |     UnaryMinus    |       Right       |                                int/float                                |                                     int/float                                     |        -1        |
|        0       |       !      |      UnaryNot     |       Right       |                                   bool                                  |                                        bool                                       |        !a        |
|        2       |       .      |   OP_AccessInner  |        Left       |                                                                         |                                                                                   |      rgb.red     |
|        2       |      ->      | OP_AccessProperty |        Left       |                                                                         |                                                                                   |     stream->d    |
|        5       |       *      |    OP_Multiply    |        Left       |                          int/float * int/float                          |                        int if both are int, otherwise float                       |        1*2       |
|        5       |       /      |     OP_Divide     |        Left       |                          int/float / int/float                          |                        int if both are int, otherwise float                       |        2/1       |
|        5       |       %      |       OP_Mod      |        Left       |                                int % int                                |                                        int                                        |        7%2       |
|        6       |       +      |       OP_Add      |        Left       | int/float + int/float or string + string or array + ANY or ANY + array | int if inputs are int, float if inputs contain float, array if inputs contain array |        1+2       |
|        6       |       -      |      OP_Minus     |        Left       |                          int/float - int/float                          |                        int if both are int, otherwise float                       |        2-1       |
|        7       |      <<      |    OP_LeftShift   |        Left       |                                int << int                               |                                        int                                        |      0b01<<5     |
|        7       |      >>      |   OP_RightShift   |        Left       |                                int >> int                               |                                        int                                        |    0b10000>>2    |
|        9       |       >      |     OP_Greater    |        Left       |                        int > int or float > float                       |                                        bool                                       |     0.1 > 0.2    |
|        9       |       <      |      OP_Less      |        Left       |                              save as above                              |                                   save as above                                   |     0.1 < 0.2    |
|        9       |      >=      |    OP_GreaterEq   |        Left       |                              save as above                              |                                   save as above                                   |    0.1 >= 0.2    |
|        9       |      <=      |     OP_LessEq     |        Left       |                              save as above                              |                                   save as above                                   |    0.1 <= 0.2    |
|       10       |      ==      |    OP_LogicalEq   |        Left       |                         inputs must be same type                        |                                        bool                                       |      a == b      |
|       10       |      !=      |  OP_LogicalNotEq  |        Left       |                              save as above                              |                                   save as above                                   |      a != b      |
|       11       |       &      |     OP_BitAnd     |        Left       |                                   int                                   |                                        int                                        |  0b0101 & 0b1010 |
|       12       |       ^      |     OP_BitXor     |        Left       |                                   int                                   |                                        int                                        |  0b0101 ^ 0b1010 |
|       13       |      \|      |      OP_BitOr     |        Left       |                                   int                                   |                                        int                                        | 0b0101 \| 0b1010 |
|       14       |      &&      |   OP_LogicalAnd   |        Left       |                               bool && bool                              |                                        bool                                       |      a && b      |
|       15       |     \|\|     |    OP_LogicalOr   |        Left       |                              bool \|\| bool                             |                                        bool                                       |     a \|\| b     |

## Scope
