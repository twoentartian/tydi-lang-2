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

## Expression and Term
