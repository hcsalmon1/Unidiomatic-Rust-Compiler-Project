mod core;
mod format;
mod parse;
mod convert;

use crate::core::errors::ConvertError;
use crate::parse::parsing;
use crate::core::debugging;
use crate::core::errors;
use crate::core::structs;
use crate::core::structs::ASTNode;
use crate::format::ast;
use errors::AstError;
use errors::ParseError;
use structs::Token;

fn convert_code(code:&String) {

    println!("Input code:\n\t{}\n", code);

    let mut parse_error:i32 = ParseError::None;
    let tokens:Vec<Token> = parsing::parse_to_tokens(code, &mut parse_error);
    if parse_error != ParseError::None {
        println!("Error: {}", errors::parse_error_to_string(parse_error));
        return;
    }

    let mut ast_error:i32 = AstError::None;
    let ast_nodes:Vec<ASTNode> = ast::build_asts(&tokens, code, &mut ast_error);
    if ast_error != AstError::None {
        println!("Error: {}", errors::ast_error_to_string(ast_error));
        debugging::print_tokens(&tokens);
        return;
    }

    let mut convert_error:i32 = ConvertError::None;
    let code:String = convert::go_convert::convert(&ast_nodes, code, &mut convert_error);
    if convert_error != ConvertError::None {
        println!("Error: {}", errors::ast_error_to_string(convert_error));
        debugging::print_tokens(&tokens);
        debugging::print_ast_nodes(&ast_nodes);
        return;
    }

    println!("\nGenerated Code:\n{}", code);
    debugging::print_tokens(&tokens);
    debugging::print_ast_nodes(&ast_nodes);
}

fn main() {
    let code:String = 
    "fn void main() {
        println(\"Hello world!\");    
    }".to_string();
    convert_code(&code);
}
