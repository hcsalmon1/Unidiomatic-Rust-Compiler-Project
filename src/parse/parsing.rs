use crate::core::enums;
use crate::core::errors;
use crate::core::printing;
use crate::core::structs;
use crate::core::structs::empty_token;
use crate::parse::parse_utils;
use enums::TokenType;
use errors::ParseError;
use structs::ParseData;
use structs::Token;

pub fn parse_to_tokens(code: &String, parse_error: &mut i32) -> Vec<Token> {
    print!("\t{}Parsing{}\t\t\t\t", printing::GREY, printing::RESET);

    let token_list: Vec<Token> = Vec::new();

    let code_as_bytes: &[u8] = code.as_bytes();

    let mut parse_data = ParseData {
        token_list,
        code: code_as_bytes,
        last_token: TokenType::Na,
        character_index: 0,
        line_count: 0,
        char_count: 0,
        was_comment: false,
    };

    if parse_data.code.len() == 0 {
        *parse_error = ParseError::Code_Length_Is_Zero;
        return parse_data.token_list;
    }

    let string_length: usize = parse_data.code.len();

    while parse_data.character_index < string_length {
        process_character(&mut parse_data, parse_error);
    }
    print!("{}Done{}\n", printing::CYAN, printing::RESET);
    return parse_data.token_list;
}

fn should_skip(parse_data: &mut ParseData) -> bool {
    parse_data.char_count += 1;
    if parse_data.last_token != TokenType::Na {
        let last_token: i32 = parse_data.last_token;

        if last_token == TokenType::Comment {
            parse_data.was_comment = true;
        }
    }

    let current_char: char = parse_data.code[parse_data.character_index] as char;

    if current_char == '\n' {
        if parse_data.was_comment == true {
            parse_data.token_list.push(Token {
                text: "".to_string(),
                token_type: TokenType::EndComment,
                line_number: parse_data.line_count,
                char_number: parse_data.char_count,
            });
            parse_data.was_comment = false;
        }
        parse_data.line_count += 1;
        parse_data.char_count = 0;
        parse_data.character_index += 1;
        return true;
    }
    let is_special_char: bool =
        current_char == '\r' || current_char == '\t' || current_char == ' ' || current_char == '\\';

    if is_special_char == true {
        parse_data.character_index += 1;
        return true;
    }
    return false;
}

fn process_character(parse_data: &mut ParseData, parse_error: &mut i32) {
    if should_skip(parse_data) == true {
        return;
    }

    let previous_character_index: usize = parse_data.character_index;
    let token: Token = get_token(parse_data, parse_error);

    if previous_character_index == parse_data.character_index {
        parse_data.character_index += 1;
    }

    parse_data.token_list.push(token.clone());
    parse_data.last_token = token.token_type;
}

fn get_token(parse_data: &mut ParseData, parse_error: &mut i32) -> Token {
    let current_char: char = parse_data.code[parse_data.character_index] as char;

    if current_char == '"' {
        return read_string(parse_data, parse_error);
    }
    if current_char == '\'' {
        return read_char(parse_data, parse_error);
    }
    if parse_utils::is_operator(current_char) {
        return read_operator(parse_data);
    }
    if parse_utils::is_separator(current_char) {
        return read_separator(parse_data);
    }

    return read_word(parse_data);
}

fn read_string(parse_data: &mut ParseData, parse_error: &mut i32) -> Token {
    let mut text_builder: String = String::new();

    // go past the '"'
    parse_data.character_index += 1;

    while parse_data.character_index < parse_data.code.len() {
        let current_char: char = parse_data.code[parse_data.character_index] as char;

        if current_char == '"' {
            parse_data.character_index += 1;
            let text: String = text_builder.clone();

            return Token {
                text,
                token_type: TokenType::StringValue,
                line_number: parse_data.line_count,
                char_number: parse_data.char_count,
            };
        }

        text_builder.push(current_char);
        parse_data.character_index += 1;
    }

    *parse_error = ParseError::Unterminated_String;
    return empty_token();
}

fn read_separator(parse_data: &mut ParseData) -> Token {
    let mut character: String = String::new();

    let current_char: char = parse_data.code[parse_data.character_index] as char;
    character.push(current_char);

    parse_data.character_index += 1;

    return Token {
        text: character.clone(),
        token_type: parse_utils::get_token_type(&character),
        line_number: parse_data.line_count,
        char_number: parse_data.char_count,
    };
}

fn read_char(parse_data: &mut ParseData, parse_error: &mut i32) -> Token {
    parse_data.character_index += 1;
    if parse_data.character_index >= parse_data.code.len() {
        *parse_error = ParseError::Unexpected_Value;
        return empty_token();
    }

    let mut char_value: String = String::new();
    let current_char: char = parse_data.code[parse_data.character_index] as char;

    char_value.push(current_char);

    parse_data.character_index += 1;

    if parse_data.character_index >= parse_data.code.len() {
        *parse_error = ParseError::Unexpected_Value;
        return structs::empty_token();
    }

    let current_char: char = parse_data.code[parse_data.character_index] as char;

    if current_char != '\'' {
        *parse_error = ParseError::Unterminated_Char;
        return structs::empty_token();
    }
    parse_data.character_index += 1;

    return Token {
        text: char_value,
        token_type: TokenType::CharValue,
        line_number: parse_data.line_count,
        char_number: parse_data.char_count,
    };
}

fn read_operator(parse_data: &mut ParseData) -> Token {
    let mut text_builder: String = String::new();

    let current_char: char = parse_data.code[parse_data.character_index] as char;

    text_builder.push(current_char);

    parse_data.character_index += 1;

    // Lookahead for compound operators like "==", "!="
    if parse_data.character_index < parse_data.code.len() {
        let next_char: char = parse_data.code[parse_data.character_index] as char;

        if parse_utils::is_operator(next_char) {
            text_builder.push(next_char);
            parse_data.character_index += 1;
        }
    }

    return Token {
        text: text_builder.clone(),
        token_type: parse_utils::get_token_type(&text_builder),
        line_number: parse_data.line_count,
        char_number: parse_data.char_count,
    };
}

fn read_word(parse_data: &mut ParseData) -> Token {
    let mut text_builder: String = String::new();

    while parse_data.character_index < parse_data.code.len() {
        let current_char: char = parse_data.code[parse_data.character_index] as char;

        if parse_utils::is_letter_or_digit(current_char) || current_char == '_' {
            text_builder.push(current_char);
            parse_data.character_index += 1;
        } else {
            break;
        }
    }

    let text_builder_ref: String = text_builder;
    let token_type: i32 = parse_utils::get_token_type(&text_builder_ref);

    return Token {
        text: text_builder_ref,
        token_type: token_type,
        line_number: parse_data.line_count,
        char_number: parse_data.char_count,
    };
}
