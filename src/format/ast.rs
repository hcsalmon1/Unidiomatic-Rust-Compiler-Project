use crate::core::debugging;
use crate::core::enums;
use crate::core::errors;
use crate::core::printing;
use crate::core::structs;
use crate::format::ast_functions;

use enums::ASTNodeType;
use enums::TokenType;
use errors::AstError;
use structs::ASTData;
use structs::ASTNode;
use structs::Token;

pub fn build_asts(token_list: &Vec<Token>, code: &String, format_error: &mut i32) -> Vec<ASTNode> {
    
    print!("\t{}Formatting{}\t\t\t", printing::GREY, printing::RESET);

    let mut ast_nodes: Vec<ASTNode> = Vec::new();

    let mut ast_data = ASTData {
        ast_nodes: &mut ast_nodes,
        token_list: token_list,
        token_index: 0,
        error_code: AstError::None,
        error_function: String::new(),
        error_detail: String::new(),
        error_token: structs::empty_token(),
    };

    let token_count: usize = ast_data.token_list.len();

    while ast_data.token_index < token_count {
        
        let index_before: usize = ast_data.token_index;

        process_global_token_ast(&mut ast_data, false);
        if ast_data.is_error() {
            println!("{}Error{}\n", printing::RED, printing::RESET);
            debugging::print_ast_error(&ast_data, code);
            *format_error = ast_data.error_code;
            return ast_data.ast_nodes.clone();
        }

        if index_before == ast_data.token_index {
            ast_data.token_index += 1;
        }
    }

    print!("{}Done{}\n", printing::CYAN, printing::RESET);

    return ast_data.ast_nodes.clone();
}

fn process_global_token_ast(ast_data: &mut ASTData, is_const: bool) {
    ast_data.error_function = "processGlobalTokenAST".to_string();

    let first_token: Token = ast_data.get_token();

    match first_token.token_type {

        TokenType::Fn => {
            ast_functions::process_function_declaration(ast_data);
        }
        _ => {
            ast_data.error_detail  = "unimplemented type in ast ".to_string();
            ast_data.error_token = first_token;
            ast_data.error_code = AstError::UnimplementedType;
            return;
        }
    }
}

