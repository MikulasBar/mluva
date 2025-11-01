# Language

## Table of contents
- [Introduction](#introduction)
- [Data Types](#data-types)
    - [Strings](#strings)
    - [Void Type](#void-type)
- [Variables](#variables)
- [Operators](#operators)
- [Control Structures](#control-structures)
    - [If / Else Statements](#if--else-statements)
    - [While Loops](#while-loops)
- [Functions](#functions)
    - [Built-in Functions](#built-in-functions)
- [Modules](#modules)
- [Comments](#comments)
- [Conventions](#conventions)

## Introduction
Mluva is a statically typed, interpreted programming language. It's not designed to be good at anything in particular, but rather to explore language design concepts and implementation techniques.

Semi-colons are not required (but are permitted) at the end of statements. Code blocks are defined using braces `{}` just like in C.

## Data Types
For now Mluva supports only the following data types:
- **Int** (32-bit signed integers)
- **Float** (32-bit floating point numbers)
- **Bool** (true and false)
- **String** (UTF-8 encoded text)
- **Void** (represents the absence of a value)

Internally, typechecking is done at compile time, so if module is loaded from bytecode, manually edited or generated, type errors may occur at runtime.

### Strings
Strings are immutable sequences of ASCII characters. They can be created using single quotes:
```
let greeting: String = 'Hello, World!'
```
Double quotes are not supported.

Use backslash `\` as escape character to include special characters in strings:
- `\'` - single quote
- `\\` - backslash
- `\n` - newline
- `\t` - tab

### Void Type
Void can be used anywhere a type is expected, but the syntax of the language doesn't allow you to create value of type Void.(Note that the value can be created in bytecode instructions, although it's not very useful.)

## Variables
Variables can be declared in function body similarly to other C-like languages:
```
Int x = 69
String y = 'Hello there!'
```

The `let` keyword can be used instead of the type name to let the compiler infer the type:
```
let x = 42.0
let y = 'Hello, World!'
```

## Operators
Mluva supports the following operators:
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Unary: `-` (negation), `!` (logical NOT)
- Logical: `&&`, `||`

Operator precedence is similar to C-like languages. Parentheses `()` can be used to group expressions and override precedence.

## Control Structures
Mluva supports the following control structures:
- `if` / `else` statements
- `while` loops

Variables declared inside control structures have block scope.

### If / Else Statements
If statements can be used to conditionally execute code blocks.
Unlike C-like syntax, parentheses around the condition are optional.

The else block is also optional.
```
if condition {
    # code to execute if condition is true
} else {
    # code to execute if condition is false
}
```

### While Loops
While loops can be used to repeatedly execute a block of code as long as a condition is true.
Parentheses around the condition are optional.

```
while condition {
    # code to execute while condition is true
}
```

## Functions
Functions can be declared in global scope using C-like syntax:
```
Int add(Int a, Int b) {
    return a + b
}
```

Return keyword is required to return a value from a function. If the return type is Void, the return statement can be omitted.

Functions can be called using standard syntax:
```
let result: Int = add(5, 10)
```

### Built-in Functions
Mluva provides several built-in functions for common tasks:
- `print` - prints all arguments to standard output
- `assert` - checks if all arguments are true, otherwise raises runtime error
- `format` - interpolates all arguments into string and returns it

If you try to name your function the same as a built-in function, compiler will raise an error.

## Modules
Modules are basic unit of code organization in Mluva. Each module is defined in its own file with `.mv` extension.

Modules are defaultly exporting all their functions.

Modules can import other modules using the `import` keyword:
```
# This imports module from file math.mv
import math
```

Functions from imported modules can be called using colon notation:
```
import math

Void main() {
    let result = math:add(5, 10)
}
```

## Comments
Single line comments start with `#` and continue to the end of the line:
```
# This is a comment
let x = 42  # This is also a comment
```
Multi-line comments are not supported.

## Conventions
- Use `snake_case` for variable, function and module names.
- Use `Camelcase` for Type names. (Not yet applicable since there are no user-defined types.)