use crate::core::enums;
use enums::TokenType;

pub const OPERATORS: &[char] = &['+', '-', '/', '*', '|', '&', '%', '>', '<', '='];
pub const SEPARATORS: &[char] = &[
    ';', '(', ')', '{', '}', '[', ']', ',', '.', '\n', '\r', '\t', '\\',
];

// Keywords
pub const FN: &str = "fn";
pub const IF: &str = "if";
pub const ELSE: &str = "else";
pub const FOR: &str = "for";
pub const WHILE: &str = "while";
pub const RETURN: &str = "return";
pub const BREAK: &str = "break";
pub const CONTINUE: &str = "continue";
pub const PRINT: &str = "print";
pub const PRINTLN: &str = "println";
pub const TRUE: &str = "true";
pub const FALSE: &str = "false";
pub const IN: &str = "in";
pub const NEW: &str = "new";
pub const DEFER: &str = "defer";
pub const DELETE: &str = "delete";

// Types
pub const I8: &str = "i8";
pub const U8: &str = "u8";
pub const I32: &str = "i32";
pub const U32: &str = "u32";
pub const I64: &str = "i64";
pub const U64: &str = "u64";
pub const F32: &str = "f32";
pub const F64: &str = "f64";
pub const STRING: &str = "string";
pub const BOOL: &str = "bool";
pub const CHAR: &str = "char";
pub const VOID: &str = "void";
pub const CONST: &str = "const";
pub const INT: &str = "int";
pub const USIZE: &str = "usize";

// Operators
pub const PLUS: &str = "+";
pub const PLUS_PLUS: &str = "++";
pub const MINUS: &str = "-";
pub const MULTIPLY: &str = "*";
pub const DIVIDE: &str = "/";
pub const EQUALS: &str = "=";
pub const PLUS_EQUALS: &str = "+=";
pub const MINUS_EQUALS: &str = "-=";
pub const MULTIPLY_EQUALS: &str = "*=";
pub const DIVIDE_EQUALS: &str = "/=";
pub const GREATER_THAN: &str = ">";
pub const LESS_THAN: &str = "<";
pub const EQUALS_EQUALS: &str = "==";
pub const GREATER_THAN_EQUALS: &str = ">=";
pub const LESS_THAN_EQUALS: &str = "<=";
pub const MODULUS: &str = "%";
pub const COMMENT: &str = "//";
pub const NOT_EQUALS: &str = "!=";
pub const AND: &str = "&";
pub const AND_AND: &str = "&&";
pub const OR: &str = "|";
pub const OR_OR: &str = "||";
pub const MODULUS_EQUALS: &str = "%=";
pub const THREE_SPACES: &str = "   ";

// Parentheses and Brackets
pub const LEFT_PARENTHESIS: &str = "(";
pub const RIGHT_PARENTHESIS: &str = ")";
pub const LEFT_BRACE: &str = "{";
pub const RIGHT_BRACE: &str = "}";
pub const LEFT_SQUARE_BRACKET: &str = "[";
pub const RIGHT_SQUARE_BRACKET: &str = "]";
pub const SEMICOLON: &str = ";";
pub const COMMA: &str = ",";
pub const FULL_STOP: &str = ".";

pub fn get_token_type(input: &String) -> i32 {

    // Keywords
    if input == FN {
        return TokenType::Fn;
    }
    if input == IF {
        return TokenType::If;
    }
    if input == ELSE {
        return TokenType::Else;
    }
    if input == FOR {
        return TokenType::For;
    }
    if input == WHILE {
        return TokenType::While;
    }
    if input == RETURN {
        return TokenType::Return;
    }
    if input == BREAK {
        return TokenType::Break;
    }
    if input == CONTINUE {
        return TokenType::Continue;
    }
    if input == PRINT {
        return TokenType::Print;
    }
    if input == PRINTLN {
        return TokenType::Println;
    }
    if input == TRUE {
        return TokenType::True;
    }
    if input == FALSE {
        return TokenType::False;
    }
    if input == IN {
        return TokenType::In;
    }
    if input == DEFER {
        return TokenType::Defer;
    }
    if input == NEW {
        return TokenType::New;
    }

    // Types
    if input == U8 {
        return TokenType::u8;
    }
    if input == I8 {
        return TokenType::i8;
    }
    if input == I32 {
        return TokenType::i32;
    }
    if input == F32 {
        return TokenType::f32;
    }
    if input == F64 {
        return TokenType::f64;
    }
    if input == I64 {
        return TokenType::i64;
    }
    if input == U64 {
        return TokenType::u64;
    }
    if input == STRING {
        return TokenType::String;
    }
    if input == BOOL {
        return TokenType::Bool;
    }
    if input == CHAR {
        return TokenType::Char;
    }
    if input == VOID {
        return TokenType::Void;
    }
    if input == CONST {
        return TokenType::Const;
    }
    if input == INT {
        return TokenType::i32;
    }
    if input == USIZE {
        return TokenType::Usize;
    }

    // Operators
    if input == PLUS_PLUS {
        return TokenType::PlusPlus;
    }
    if input == PLUS {
        return TokenType::Plus;
    }
    if input == MINUS {
        return TokenType::Minus;
    }
    if input == MULTIPLY {
        return TokenType::Multiply;
    }
    if input == DIVIDE {
        return TokenType::Divide;
    }
    if input == EQUALS {
        return TokenType::Equals;
    }
    if input == PLUS_EQUALS {
        return TokenType::PlusEquals;
    }
    if input == MINUS_EQUALS {
        return TokenType::MinusEquals;
    }
    if input == MULTIPLY_EQUALS {
        return TokenType::MultiplyEquals;
    }
    if input == DIVIDE_EQUALS {
        return TokenType::DivideEquals;
    }
    if input == GREATER_THAN {
        return TokenType::GreaterThan;
    }
    if input == LESS_THAN {
        return TokenType::LessThan;
    }
    if input == EQUALS_EQUALS {
        return TokenType::EqualsEquals;
    }
    if input == GREATER_THAN_EQUALS {
        return TokenType::GreaterThanEquals;
    }
    if input == LESS_THAN_EQUALS {
        return TokenType::LessThanEquals;
    }
    if input == MODULUS {
        return TokenType::Modulus;
    }
    if input == NOT_EQUALS {
        return TokenType::NotEquals;
    }
    if input == AND {
        return TokenType::And;
    }
    if input == AND_AND {
        return TokenType::AndAnd;
    }
    if input == OR {
        return TokenType::Or;
    }
    if input == OR_OR {
        return TokenType::OrOr;
    }
    if input == MODULUS_EQUALS {
        return TokenType::ModulusEquals;
    }

    if input == COMMENT {
        return TokenType::Comment;
    }
    if input == DELETE {
        return TokenType::Delete;
    }

    // Parentheses and Brackets
    if input == LEFT_PARENTHESIS {
        return TokenType::LeftParenthesis;
    }
    if input == RIGHT_PARENTHESIS {
        return TokenType::RightParenthesis;
    }
    if input == LEFT_BRACE {
        return TokenType::LeftBrace;
    }
    if input == RIGHT_BRACE {
        return TokenType::RightBrace;
    }
    if input == LEFT_SQUARE_BRACKET {
        return TokenType::LeftSquareBracket;
    }
    if input == RIGHT_SQUARE_BRACKET {
        return TokenType::RightSquareBracket;
    }

    if input == SEMICOLON {
        return TokenType::Semicolon;
    }
    if input == COMMA {
        return TokenType::Comma;
    }
    if input == FULL_STOP {
        return TokenType::FullStop;
    }

    // Number literals
    if is_integer(input) {
        return TokenType::IntegerValue;
    }
    if is_decimal(input) {
        return TokenType::DecimalValue;
    }

    // String or Char value
    if input.contains('"') {
        return TokenType::StringValue;
    }
    if input.contains('\'') {
        return TokenType::CharValue;
    }

    return TokenType::Identifier;
}

pub fn is_operator(character: char) -> bool {
    const LENGTH: usize = OPERATORS.len();
    for i in 0..LENGTH {
        if character == OPERATORS[i] {
            return true;
        }
    }
    return false;
}

pub fn is_separator(character: char) -> bool {
    const LENGTH: usize = SEPARATORS.len();
    for i in 0..LENGTH {
        if character == SEPARATORS[i] {
            return true;
        }
    }
    return false;
}

pub fn is_integer(input: &String) -> bool {
    let length: usize = input.len();

    let chars: &[u8] = input.as_bytes();

    for i in 0..length {
        let current_char: char = chars[i] as char;
        if current_char == '-' {
            if i != 0 {
                return false;
            }
            continue;
        }
        if is_digit(current_char) == false {
            return false;
        }
    }
    return true;
}

pub fn is_decimal(input: &String) -> bool {
    let length: usize = input.len();
    let chars: &[u8] = input.as_bytes();

    for i in 0..length {
        let current_char: char = chars[i] as char;
        if current_char == '-' {
            if i != 0 {
                return false;
            }
            continue;
        }
        if current_char == '.' {
            continue;
        }
        if is_digit(current_char) == false {
            return false;
        }
    }
    return true;
}

pub fn is_letter_or_digit(char: char) -> bool {
    match char {
        'a'..='z' | 'A'..='Z' | '0'..='9' => return true,
        _ => return false,
    }
}

pub fn is_digit(char: char) -> bool {
    match char {
        '0'..='9' => return true,
        _ => return false,
    }
}
