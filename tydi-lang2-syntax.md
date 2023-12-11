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

## Alias (aka Variable)

Alias means binding an expression / a logical type / a value / a template instance to an identifier (aka variable). For example:

```cpp
a = 1 + 2;   //a is an identifier and 1 + 2 is an expression
bit_8 = Bit(8);  //bind bit_8 to logic type "Bit(8)"
```

You can also specify the type of an alias:

```cpp
a : int = 1 + 2;
```

The ```: int``` is also called type indicator. For most cases, type indicator is optional and can be one of the basic data types. Notice that the type indicator cannot be a logic types because syntax like ```t: LogicTypes ``` means declaring a logic type.

In Tydi-lang, due to the absence of a defined execution order, the values of all variables need to be determinable at compile time. Consequently, code that doesn't adhere to this requirement will lead to compile-time errors.

```cpp
i = i + 1; // the evaluation of i is dependent on i, causing infinite evaluation
i : int; // no value is given.
```

## Array

```cpp
a = [1, 2.0, 3, True, Bit(1)] //Declare an array
```
The type of an array is always an array and only its elements have specific types. For above example, a[0] is an int, a[1] is a float, etc.

## Expression, operator, and Term
An expression is a combination of terms and operators. For example: expression `1+2` has 2 terms(`1` and `2`) and 1 operator(`+`). Terms include values of basic types and variables. The precedence of Tydi-lang operators basically follows the C++ operator precedence:
| **Precedence** | **Operator** |  **Description**  | **Associativity** |                            **Allowed types**                            |                                  **Result type**                                  |    **Example**   |
|:--------------:|:------------:|:-----------------:|:-----------------:|:-----------------------------------------------------------------------:|:---------------------------------------------------------------------------------:|:----------------:|
|        0       |       -      |     UnaryMinus    |   Right to left   |                                int/float                                |                                     int/float                                     |        -1        |
|        0       |       !      |      UnaryNot     |   Right to left   |                                   bool                                  |                                        bool                                       |        !a        |
|        2       |       .      |   OP_AccessInner  |   Left to right   |                                                                         |                                                                                   |      rgb.red     |
|        2       |      ->      | OP_AccessProperty |   Left to right   |                                                                         |                                                                                   |     stream->d    |
|        5       |       *      |    OP_Multiply    |   Left to right   |                          int/float * int/float                          |                        int if both are int, otherwise float                       |        1*2       |
|        5       |       /      |     OP_Divide     |   Left to right   |                          int/float / int/float                          |                        int if both are int, otherwise float                       |        2/1       |
|        5       |       %      |       OP_Mod      |   Left to right   |                                int % int                                |                                        int                                        |        7%2       |
|        6       |       +      |       OP_Add      |   Left to right   | int/float + int/float or string + string or array + ANY or ANY \| array | int if inputs are int float if inputs contain float array if inputs contain array |        1+2       |
|        6       |       -      |      OP_Minus     |   Left to right   |                          int/float - int/float                          |                        int if both are int, otherwise float                       |        2-1       |
|        7       |      <<      |    OP_LeftShift   |   Left to right   |                                int << int                               |                                        int                                        |      0b01<<5     |
|        7       |      >>      |   OP_RightShift   |   Left to right   |                                int >> int                               |                                        int                                        |    0b10000>>2    |
|        9       |       >      |     OP_Greater    |   Left to right   |                        int > int or float > float                       |                                        bool                                       |     0.1 > 0.2    |
|        9       |       <      |      OP_Less      |   Left to right   |                              save as above                              |                                   save as above                                   |     0.1 < 0.2    |
|        9       |      >=      |    OP_GreaterEq   |   Left to right   |                              save as above                              |                                   save as above                                   |    0.1 >= 0.2    |
|        9       |      <=      |     OP_LessEq     |   Left to right   |                              save as above                              |                                   save as above                                   |    0.1 <= 0.2    |
|       10       |      ==      |    OP_LogicalEq   |   Left to right   |                         inputs must be same type                        |                                        bool                                       |      a == b      |
|       10       |      !=      |  OP_LogicalNotEq  |   Left to right   |                              save as above                              |                                   save as above                                   |      a != b      |
|       11       |       &      |     OP_BitAnd     |   Left to right   |                                   int                                   |                                        int                                        |  0b0101 & 0b1010 |
|       12       |       ^      |     OP_BitXor     |   Left to right   |                                   int                                   |                                        int                                        |  0b0101 ^ 0b1010 |
|       13       |      \|      |      OP_BitOr     |   Left to right   |                                   int                                   |                                        int                                        | 0b0101 \| 0b1010 |
|       14       |      &&      |   OP_LogicalAnd   |   Left to right   |                               bool && bool                              |                                        bool                                       |      a && b      |
|       15       |     \|\|     |    OP_LogicalOr   |   Left to right   |                              bool \|\| bool                             |                                        bool                                       |     a \|\| b     |

## Scope
Scope is a code region that contains lanauges elements such as variables, logical types, streamlets and implementations. There are two ways of declaring a scope:
- A Tydi source file inherently acts as a scope.
- Any code section enclosed within a set of braces (`{...}`) forms a distinct scope.

Example:
```cpp
//start of Scope 1
package pack1;

i = 8;
m = 8;
streamlet bypass <logic_type: type> {  //start of Scope 2
    in_port: logic_type in;
    out_port: logic_type out;
    m = i + 1;
    i = 10;
}  //end of Scope 2

impl i_bypass <logic_type: type> of bypass<logic_type> {  //start of Scope 3
    in_port => out_port;
}  //end of Scope 3

bypass_bit8 = i_bypass<pack0.bit8_stream>;
bypass_bit16 = i_bypass<pack0.bit16_stream>;
//end of Scope 1
```

In the given example, Scope 2, nested within Scope 1, is its "child scope". Child scopes can access elements from their parent scope, so the `i` in `m = i + 1` in Scope 2 refers to i = 8 in Scope 1. Different variables with the same names can be declared independently in Scope 1 and Scope 2, as they are distinct scopes. 

When resolving variables, the search begins in the scope where the request is made. If the variable is not found there, the search moves to the parent scopes. In the first example, i is resolved within its own scope. In the second example, i is resolved using the declaration in the parent scope.
```cpp
i = 8;
m = 10;
Union test {
    m = [i]* + 1;
    [i]* = 10;
}
```

```cpp
[i]* = 8;
m = 10;
Union test {
    m = [i]* + 1;
}
```

## Bit
Syntax:
```cpp
bit_8 = Bit(8);
```
**Notice**: because Bit(8) has no specific meanings to indicate its functionality, we can declare `char=Bit(8)` and `red=Bit(8)` but `char` and `red` have different meanings. Thus here `Bit(8)` will be declared as an anonymous variable and `bit_8` is an alias of that anonymous variable. With this mechanism, `char` and `red` will be resolved to two different anonymous variables.

## Logical Group (Union)
Syntax:
```cpp
#document#      //this is optional
Group {ID} [<{TEMPLATE_ARGS}>]* {    //template args are optional
    //Group elements, such as 
    bit_8: Bit(8);          //(1) logical types
    
    for i in [1,2,3]        //(2) control block
    {
        assert(i - 4 < 0);
        data: Bit(i);
    }
    
    m = n;                  //(3) declaring variables

}
```

Example:
```cpp
Group any_bit <n: int> {    
    bit_n: Bit(n);
    m = n;
    for i in [1,2,3]
    {
        assert(i - 4 < 0);
        data: Bit(i);
    }
}
```
Evaluation result for any_bit<9>:
```cpp
any_bit<9>:
    - "bit_n": Bit(9)
    - "data": Array( Bit(1), Bit(2), Bit(3) )
    - "n": 9    //declared by template arg
    - "m": 9
```

## Stream
Syntax:
```cpp
Stream ( {LOGICAL_TYPE} [, <STREAM_OPTION>]* )
```

Example:
```cpp
char_string_stream = Stream(char_8, d=1); //both dimension=1 and d=1(the abbr version) are ok
```
Available options:
|     option    | abbr |                    candidate                   | default value |
|:-------------:|:----:|:----------------------------------------------:|:-------------:|
|   dimension   |   d  |                       int                      |       1       |
|   user_type   |   u  |            non-stream logical types            |      Null     |
|   throughput  |   t  |                      float                     |      1.0      |
| synchronicity |   s  | string("Sync","Flatten","Desync","FlatDesync") |     "Sync"    |
|   complexity  |   c  |                    int(1~7)                    |       1       |
|   direction   |   r  |           string("Forward","Reverse")          |   "Forward"   |
|      keep     |   x  |                      bool                      |     false     |


## Streamlet
In Tydi-lang, streamlet describes the port of a component. Similar to the "entity" concept in VHDL.

Syntax:
```cpp
#document#      //this is optional
"streamlet" {ID}  [ < {TEMPLATE_ARGS} [,{TEMPLATE_ARGS}]* > ]?  {ATTRIBUTE}* {
    //in the streamlet scope, you can define logical types, variables as mentioned before.
    //in addition, you can also define a port
    #document#  //optional
    {ID} ":" {LogicalType} {PortDirection} {PortTimeDomain}? {ATTRIBUTE}*
    //PortDirection can only be in or out
    //PortTimeDomain should be "/" + a string literal or a string variable
}
```


Example:
```cpp
streamlet bypass <logic_type: type> {
    in_port: logic_type in \time_domain_var;
    out_port: logic_type out \"100MHz" ;
}
```

To define a port array:
```cpp
streamlet bypass <logic_type: type> {
    in_port: logic_type in \time_domain_var;
    for i in [0,1] {
        out_port: logic_type out \"100MHz" ; //use subscription to access an element: out_port[0]
    }
}
```

## Implementation
Implementation describes the internal layout (connections, sub components, etc) of a streamlet.

Syntax:
```cpp
#document#      //this is optional
"impl" {ID}  [ < {TEMPLATE_ARGS} [,{TEMPLATE_ARGS}]* > ]? "of" {ID} [ < {Exp} [, {Exp}]* > ]?  {ATTRIBUTE}* {
    //The first ID is the name of the implementation, the second ID is the derived streamlet ID.

    //in the implementation scope, you can define variables as mentioned before.
    //in addition, you can also define an instance:
    #document#  //optional
    "instance" {ID} ( {Exp} ) {ATTRIBUTE}*
    //Exp should be an implementation name, indicating using another implementation here.

    #document#  //optional
    {Exp} ~ "=>" {Exp} {NetName}? ATTRIBUTE* //attribute: NoTypeCheck
    //the two Exps should be port name or instance_name.port_name.
    //NetName is optional
}
```

Example:
```cpp
streamlet bypass <logic_type: type> {
    in_port: logic_type in;
    out_port: logic_type out;
}

impl i_bypass <logic_type: type> of bypass<logic_type> {
    in_port => out_port;
}
```

## Template
Template can be applied on Group, Union, streamlet and implementation. Template arguements can be basic values: int(`x:int`), float(`x:float`), bool(`x:bool`), string(`x:string`), clockdomain(`x:clock`), logical types(`x:type`) and streamlet(`x:streamlet`)


## Attribute (under implementation)
Attribute is used to set some special properties of a componet, port, connection.
```cpp
impl i_bypass <logic_type: type> of bypass<logic_type> @NoTemplateExpansion {   //Do not do template expansion
    in_port => out_port;
}
```

current plan for implementation:
`@NoTemplateExpansion`
`@NoStrictTypeChecking`

