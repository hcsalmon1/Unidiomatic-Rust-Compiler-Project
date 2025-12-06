use crate::core::enums;
use crate::core::structs;
use enums::TokenType;
use structs::Token;

pub fn is_type_token(token: Token) -> bool {
    if is_var_type(token.token_type) == true {
        return true;
    }
    if token.token_type == TokenType::Identifier {
        return true;
    }
    return false;
}

pub fn is_binary_operator_bool(tokenType: i32) -> bool {
    return tokenType == TokenType::Plus
        || tokenType == TokenType::Minus
        || tokenType == TokenType::Multiply
        || tokenType == TokenType::Divide
        || tokenType == TokenType::AndAnd
        || tokenType == TokenType::OrOr
        || tokenType == TokenType::LessThan
        || tokenType == TokenType::LessThanEquals
        || tokenType == TokenType::GreaterThan
        || tokenType == TokenType::GreaterThanEquals
        || tokenType == TokenType::EqualsEquals
        || tokenType == TokenType::NotEquals;
}

pub fn is_var_type(token_type: i32) -> bool {
    return token_type == TokenType::Bool
        || token_type == TokenType::Char
        || token_type == TokenType::Int
        || token_type == TokenType::f32
        || token_type == TokenType::f64
        || token_type == TokenType::i16
        || token_type == TokenType::i32
        || token_type == TokenType::i64
        || token_type == TokenType::i8
        || token_type == TokenType::u16
        || token_type == TokenType::u32
        || token_type == TokenType::u64
        || token_type == TokenType::Usize
        || token_type == TokenType::u8
        || token_type == TokenType::String
        || token_type == TokenType::Void;
}

pub fn is_integer_var_type(token_type: i32) -> bool {
    return token_type == TokenType::Int
        || token_type == TokenType::i16
        || token_type == TokenType::i32
        || token_type == TokenType::i64
        || token_type == TokenType::i8
        || token_type == TokenType::u16
        || token_type == TokenType::u32
        || token_type == TokenType::u64
        || token_type == TokenType::Usize
        || token_type == TokenType::u8;
}

pub fn get_precedence_bool(token_type:i32) -> usize {
    
    match token_type {
        TokenType::OrOr => return 1, // lowest
        TokenType::AndAnd => return 2,
        TokenType::EqualsEquals | TokenType::NotEquals => return 3,
        TokenType::LessThan | TokenType::GreaterThan | TokenType::LessThanEquals | TokenType::GreaterThanEquals => return 4,
        TokenType::Plus | TokenType::Minus => return 5,
        TokenType::Multiply | TokenType::Divide => return 6,
        _ => return 0,
    }
}