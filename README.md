# Unidiomatic-Rust-Compiler-Project
I have a programming style that is the opposite of Rust's, let's see how incompitable they are.

Rules:
-No tagged unions, real enums  
-No iterators if possible  
-No traits if possible  
-No function call chains  
-No inferred types  
-Procedural code  

<b>Questions and Answers:</b>

<b>Are you trolling?</b>

Yes. But honestly if someone forced me to write code in Rust, I would probably write it this way.  

<b>Why No Tagged Unions?</b>

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
#[derive(Debug, PartialEq, Eq)]
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

In this project I banned this type completely. The original code was in Zig and C# and had no tagged unions.  
To create my own namespaced named constants to integers, I used structs and created a gui to generate them for me:  

This:  
```rust
enum ParseError {
    None,
    Code_Length_Is_Zero,
    Unterminated_String,
    Unexpected_Value,
    Unterminated_Char,
}
```
Autogenerates this for me with the gui tool:  
```rust
pub struct ParseError;
impl ParseError {
    pub const None:i32 = 0;
    pub const Code_Length_Is_Zero: i32 = 1;
    pub const Unterminated_String: i32 = 2;
    pub const Unexpected_Value: i32 = 3;
    pub const Unterminated_Char: i32 = 4;
}

pub fn parse_error_to_string(error: i32) -> &'static str {
    match error {
        ParseError::Code_Length_Is_Zero => "Code_Length_Is_Zero",
        ParseError::Unterminated_String => "Unterminated_String",
        ParseError::Unexpected_Value => "Unexpected_Value",
        ParseError::Unterminated_Char => "Unterminated_Char",
        _ => "Unknown",
    }
}
```
Then I copy and paste it into my project and I have proper enums without derive spam.  

<b>Why no iterators?</b>

This project is not suited at all to using them, even though they are everywhere in Rust.  
The parsing section alone is not suited to iterators at all.  

The parsing logic:  
-Loop through each character  
-If you find a letter that could be part of type  
    Loop until you find the end that type and add it to the list  
-In the initial loop, skip the characters just added in the inner loop  

The entire parsing logic would be extremely difficult

I'm not against the idea of iterators, I just want the option to choose.  
Zig gives you that option.
```zig
const list:ArrayList<i32> = getArrayList();
for (list) |element| { //automates an iterator in the background
    print("Element: {}\n", .{element});
}
```
vs
```zig
const list:ArrayList<i32> = getArrayList();
const list_count:usize = list.items.len;
for (0..list_count) |i| {
    const element:i32 = list.items[i];
    print("Element: {}\n", .{element});
}
```
Rust makes this very difficult to do with certain types.  
It's like they don't trust you to ever access the index of something directly.  
The thing is, in the above example I am looping against the length of the ArrayList and never mutate it, how get it go out of bounds?  

Iterators are great when you only need to loop through a collection in order but they are extremely cumbersome if you need to any advances iteration.  
What if you want to go back to the start of the loop if you meet a certain condition? Good luck doing that with a Rust iterator.  

Another reason I don't like them, is I don't like inferred types. It's impossible to use iterators without inferred types in Rust.  
The only way is to shadow, like this:  

```rust
for token in tokens {
   let token:Token = token;
}
```

<b> Why no inferred types?</b>

