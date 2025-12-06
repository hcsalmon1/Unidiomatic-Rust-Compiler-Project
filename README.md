# Unidiomatic-Rust-Compiler-Project
I have a programming style that is the opposite of Rust's, let's see how incompitable they are.

Rules:  
-No tagged unions, real enums  
-No iterators if possible  
-No traits if possible  
-No function call chains  
-No inferred types  
-Procedural code  

<b>__Questions and Answers:__</b>

<b>__Are you trolling?__</b>

Yes. But honestly if someone forced me to write code in Rust, I would probably write it this way.  

<b>__Why No Tagged Unions?__</b>

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
Zig gives me what I want by default. You can compare and print them innately.  
Rust requires this derive spam to force a tagged union into integers.  

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

I do think tagged unions are useful, if you actually want to use them for a task.  
When I just want status codes, why would I need them? And why am I forced to write derive spam to make them usable?  

<b>__Why no iterators?__</b>

This project is not suited at all to using them, even though they are everywhere in Rust.  
The parsing section alone is not suited to iterators at all.  

The parsing logic:  

    -Loop through each character  
    -If you find a letter that could be part of type  
        Loop until you find the end that type and add it to the list  
    -In the initial loop, skip the characters just added in the inner loop  

The entire parsing logic would be extremely difficult with iterators.  
You'd need an iterator inside an iterator and skip the unwanted characters.  
Massive nightmare.  

I'm not against the idea of iterators, I just want the option to choose.  
Zig gives you that option.
```zig
const list:ArrayList(i32) = getArrayList();
for (list) |element| { //automates an iterator in the background
    print("Element: {}\n", .{element});
}
```
vs
```zig
const list:ArrayList(i32) = getArrayList();
const list_count:usize = list.items.len;
for (0..list_count) |i| {
    const element:i32 = list.items[i];
    print("Element: {}\n", .{element});
}
```
Rust makes this very difficult to do with certain types.  
It's like they don't trust you to ever access the index of something directly.  
The thing is, in the above example I am looping against the length of the ArrayList and never mutate it, how can it go out of bounds?  

Iterators are great when you only need to loop through a collection in order, but they are extremely cumbersome if you need to do any advanced iteration.  
What if you want to go back to the start of the loop if you meet a certain condition? Good luck doing that with a Rust iterator.  

Another reason I don't like them is: I don't like inferred types. It's impossible to use iterators without inferred types in Rust.  
The only way is to shadow, like this:  

```rust
for token in tokens {
   let token:Token = token;
}
```

<b>__Why no inferred types?__</b>

They are the worst modern trend in programming.  
1. They are lazy
2. They make your code less readable
3. They force you to use an IDE and hover constantly
4. Makes code on Github horrible to read
5. It will take you longer to understand your old code because you have no context
6. It means you understand less of the language because you never learn the actual types

Most people will say things like:
-It's so verbose, just infer it  
-Just use an IDE and hover  

I code for my future self. When I haven't seen a project in 6 months and can't remember anything,  
what do you think I would prefer to see:
1. Cryptic short names and inferred types  
   or  
2. Detailed names with explicit types  

People will also say: But it's idiomatic.  

Do you think I care about that?  
That's just an argument from majority or popularity, a logical fallacy for a reason.  

From my experience, I understand my code way faster when I write good names and always write the type.  

<b>__Why no function call chains?__</b>

I really shouldn't need to explain this one.  
Why is this considered good?  

Standard Rust looks like:  

```Rust
fn print_employees_over_50(employees:&Vec<Employee>) {

    let mut over_50: Vec<&Employee> = employees
        .iter()
        .filter(|e| e.age() > 50)
        .collect();

    over_50.sort_by_key(|e| -(e.age()));

    println!("Employees over 50 (oldest first):");
    for e in over_50 {
        println!("{} - {} years old", e.name, e.age());
    }
}
```
This will create a load of hidden types, have hidden iterations, have hidden allocations etc.  
It's like a black box of function calls. C# does this with LINQ too and I hate it there as well.  
I would just create reusable functions and write this:  
```Rust
fn print_employees_over_50_proc(employees:&Vec<Employee>) {

    let employees_over_50:Vec<&Employee> = get_employee_over_age(employees, 50);
    let sorted_employees_over_50:Vec<&Employee> = sort_employees_by_age(&employees_over_50);
    for employee in sorted_employees_over_50 {
        println!("{} - {} years old", employee.name, employee.age());
    }
}
```
This is just a style I despise. I want one instruction per line, not about 5 with hidden iterators.  
Also, look at my Rust in this project. Have you seen more readable Rust code in your life?  

<b>Why no traits?</b>

These are just like interfaces and I never use them in my projects.  
I didn't write them in the original project and they don't exist in Zig, so why would I use them here?  

I was forced to use them in some places because wouldn't shut up if I didn't.  

<b>Why Procedular</b>

It's the most readable style possible. Nothing is hidden, everything is in logical order.  
You don't need to through through mountains of Russian dolls to find the actual code.  
You also know exactly what your code is doing and don't hide allocations and looping inside function-call-ception.  

