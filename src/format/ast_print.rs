use crate::core::enums;
use crate::core::errors;
use crate::core::structs;
use crate::format::ast_utils;
use enums::ASTNodeType;
use enums::TokenType;
use errors::AstError;
use structs::ASTData;
use structs::ASTNode;
use structs::Token;

pub fn process_print(ast_data: &mut ASTData, new_line: bool) -> ASTNode {
    ast_data.error_function = "processPrint".to_string();

    let mut print_node: ASTNode = structs::get_default_node();

    let child_list: Vec<ASTNode> = Vec::new();

    print_node.children = child_list;

    if new_line == true {
        print_node.node_type = ASTNodeType::Println;
    } else {
        print_node.node_type = ASTNodeType::Print;
    }

    let token: Token = ast_data.get_token();
    if ast_data.is_error() {
        return print_node;
    }

    let token_pointer: *mut Token = structs::create_raw_pointer(token);

    print_node.token = token_pointer;
    //skip print
    ast_data.token_index += 1;

    if ast_data.expect_type(TokenType::LeftParenthesis, "missing expected '(' in print") == false {
        return print_node;
    }
    ast_data.token_index += 1;

    let index_before: usize = ast_data.token_index;

    ast_utils::fill_node_in_brackets(ast_data, &mut print_node, ASTNodeType::PrintExpression);

    let is_empty_print: bool = index_before == ast_data.token_index;

    if is_empty_print == true {
        if new_line == false {
            ast_data.error_detail = "empty print function".to_string();
            ast_data.error_code = AstError::UnexpectedType;
            return print_node;
        }
    }

    if ast_data.expect_type(TokenType::RightParenthesis, "missing expected ')' in print") == false {
        return print_node;
    }
    ast_data.token_index += 1;
    if ast_data.expect_type(TokenType::Semicolon, "missing expected ';' in print") == false {
        return print_node;
    }
    ast_data.token_index += 1;

    return print_node;
}
