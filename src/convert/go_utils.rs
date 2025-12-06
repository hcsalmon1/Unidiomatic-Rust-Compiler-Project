
use crate::core::structs;
use structs::Token;
use crate::core::enums;
use enums::TokenType;

pub fn convert_to_go_type(token:Token) -> &'static str {
    match token.token_type {
            TokenType::i8 | TokenType::u8 => return "int8",
            TokenType::i16 | TokenType::u16 => return "int16",
            TokenType::Int | TokenType::i32 | TokenType::u32 => return "int",
            TokenType::i64 | TokenType::u64 | TokenType::Usize => return "int64",
            TokenType::f32 => return "float32",
            TokenType::f64 => return "float64",
            TokenType::String => return "string",
            TokenType::Char => return "byte",
            TokenType::Bool => return "bool",
            TokenType::Void => return "",
            _ => return "Unknown",
    }
}

pub fn convert_type_to_go_type(token_type:i32) -> &'static str {
    match token_type {
            TokenType::i8 | TokenType::u8 => return "int8",
            TokenType::i16 | TokenType::u16 => return "int16",
            TokenType::Int | TokenType::i32 | TokenType::u32 => return "int",
            TokenType::i64 | TokenType::u64 | TokenType::Usize => return "int64",
            TokenType::f32 => return "float32",
            TokenType::f64 => return "float64",
            TokenType::String => return "string",
            TokenType::Char => return "byte",
            TokenType::Bool => return "bool",
            TokenType::Void => return "",
            _ => return "Unknown",
    }
}