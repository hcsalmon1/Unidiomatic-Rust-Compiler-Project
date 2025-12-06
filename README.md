# Unidiomatic-Rust-Compiler-Project
I have a programming style that is the opposite of Rust's, let's see how incompitable they are.

Rules:
-No tagged unions, real enums  
-No iterators if possible  
-No traits if possible  
-No function call chains  
-No inferred types  
-Procedural code  

Questions and Answers:

<b>Why No Tagged Unions</b>

Rust doesn't have enums, instead they have tagged unions. Types that can carry anything in each element.  
Here are 5 structs, normal for usual AST compilers:  

```rust
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<AstNode>,
}

pub struct IfStatement {
    pub condition: AstNode,
    pub then_branch: Vec<AstNode>,
    pub else_branch: Option<Vec<AstNode>>,
}

pub struct VarDecl {
    pub name: String,
    pub value: AstNode,
}

pub struct Print {
    pub value: AstNode,
}

pub struct Reassignment {
    pub name: String,
    pub value: AstNode,
}
```

Then you would put them in a tagged union:
```rust
pub enum AstNode {
    FunctionDecl(FunctionDecl),
    IfStatement(IfStatement),
    VarDecl(VarDecl),
    Print(Print),
    Reassignment(Reassignment),
}
```
For some weird reason, Rust called these things 'enums'.  
No other language would refer to these things as that. They are tagged unions.  
You can force a tagged union into pretending to be an enum.  
You do this like so:  
```rust
enum ParseError {
    None,
    CodeLengthIsZero,
    UnterminatedString,
    UnexpectedValue,
    UnterminatedChar,
}
```
The problem with this is that it's completely useless to me.  
```rust
let parse_error:ParseError = ParseError::None;
...
if parse_error != ParseError::None {  //error
...
```
This will give you a compile error:  

    binary operator '!=' cannot be applied to ParseError

An enum should just be integers. Why can't you compare integers?  
This is because it's not an enum, it's a tagged union, so could have any type.  
You have to tell the compiler what it is EVERY TIME.  
You did this like so:  
```rust
#[repr(i32)]
#[Derive(Debug, PartialEq, Eq)]
enum ParseError {
    None,
    CodeLengthIsZero,
    UnterminatedString,
    UnexpectedValue,
    UnterminatedChar,
}
```
This tells the compiler:
-It represent i32 values
-Write a function to print the names
-Write a function to compare the values
This is exactly the same as doing this though:
```rust
enum ParseError {
    None(i32),
    CodeLengthIsZero(i32),
    UnterminatedString(i32),
    UnexpectedValue(i32),
    UnterminatedChar(i32),
}
```
There are tagged unions where the nested type is an i32.
What's my problem with this?
90% of the time I don't want a tagged union, I want namespaced names constants to integers, i.e. actual enums.
Rust gives me a type that could hold anything in each element, when all I want is integers.

I don't want that and to get what I want, I have to spam #derive above every definition:
```zig
Zig:
const Color = enum {
    Red,
    Green,
    Blue,
};

const Direction = enum {
    North,
    South,
    East,
    West,
};

const FileError = enum {
    NotFound,
    PermissionDenied,
    Corrupted,
};

const LogLevel = enum {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
};

const InputAction = enum {
    MoveLeft,
    MoveRight,
    Jump,
    Attack,
    Pause,
};
```
```rust
Rust:
#[repr(i32)]
#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}
#[repr(i32)]
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}
#[repr(i32)]
#[derive(Debug, PartialEq, Eq)]
enum FileError {
    NotFound,
    PermissionDenied,
    Corrupted,
}
#[repr(i32)]
#[derive(Debug, PartialEq, Eq)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
};
#[repr(i32)]
#[derive(Debug, PartialEq, Eq)]
enum InputAction {
    MoveLeft,
    MoveRight,
    Jump,
    Attack,
    Pause,
};
```
This definitely isn't a fundamental design flaw. No, not at all.  
Writing all of this boilerplate is a feature.  
