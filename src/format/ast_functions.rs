use crate::core::enums;
use crate::core::errors;
use crate::core::structs;
use crate::format::ast_expressions;
use crate::format::ast_print;
use enums::ASTNodeType;
use enums::TokenType;
use errors::AstError;
use structs::ASTData;
use structs::ASTNode;
use structs::Token;

pub fn process_function_declaration(ast_data: &mut ASTData) {
    ast_data.error_function = "processFunctionDeclaration".to_string();

    if ast_data.increment_index() == false {
        return;
    }

    let type_node: ASTNode = ast_expressions::create_complex_declarations(ast_data);
    if ast_data.is_error() {
        return;
    }

    //var name expected
    let var_name_token: Token = ast_data.get_token();
    if ast_data.is_error() {
        return;
    }

    if var_name_token.token_type != TokenType::Identifier {
        ast_data.error_detail = "Missing expected function name".to_string();
        ast_data.error_code = AstError::MissingExpectedType;
    }
    if ast_data.increment_index() == false {
        return;
    }
    if ast_data.expect_type(TokenType::LeftParenthesis, "Missing expected '('") == false {
        return;
    }
    if ast_data.increment_index() == false {
        return;
    }

    //get parameters
    let parameters_node: ASTNode = ast_expressions::fill_parameters(ast_data);
    if ast_data.is_error() {
        return;
    }

    //expect ')'
    if ast_data.expect_type(TokenType::RightParenthesis, "Missing expected ')'") == false {
        return;
    }
    if ast_data.increment_index() == false {
        return;
    }
    if ast_data.expect_type(TokenType::LeftBrace, "Missing expected '{'") == false {
        return;
    }
    ast_data.token_index += 1;

    let function_body_node: ASTNode = build_body_block(ast_data, ASTNodeType::FunctionBody);
    if ast_data.is_error() {
        return;
    }

    if ast_data.expect_type(TokenType::RightBrace, "Missing expected '}'") == false {
        return;
    }
    ast_data.token_index += 1;

    let box_var_name_token: Box<Token> = Box::new(var_name_token);
    let box_function_body: Box<ASTNode> = Box::new(function_body_node);
    let box_type: Box<ASTNode> = Box::new(type_node);
    let box_parameters: Box<ASTNode> = Box::new(parameters_node);

    let mut function_node: ASTNode = structs::get_default_node();
    function_node.node_type = ASTNodeType::FunctionDeclaration;
    function_node.token = Box::into_raw(box_var_name_token);
    function_node.left = Box::into_raw(box_type);
    function_node.middle = Box::into_raw(box_parameters);
    function_node.right = Box::into_raw(box_function_body);

    ast_data.ast_nodes.push(function_node);
}

pub fn build_body_block(ast_data: &mut ASTData, node_type: i32) -> ASTNode {
    ast_data.error_function = "buildBodyBlock".to_string();

    let token_count: usize = ast_data.token_list.len();

    let mut block_node: ASTNode = structs::get_default_node();

    let child_list: Vec<ASTNode> = Vec::new();

    block_node.children = child_list;
    block_node.node_type = node_type;

    while ast_data.token_index < token_count {
        let index_before: usize = ast_data.token_index;

        let token: Token = ast_data.get_token();
        if token.token_type == TokenType::RightBrace {
            break;
        }
        process_function_token_ast(
            ast_data,
            token,
            &mut block_node,
            false, //is const
        );

        if ast_data.is_error() {
            return block_node;
        }

        if index_before == ast_data.token_index {
            ast_data.token_index += 1;
        }
    }

    return block_node;
}

pub fn process_function_token_ast(ast_data:&mut ASTData, first_token:Token, block_node:&mut ASTNode, is_const:bool) {

    ast_data.error_function = "processFunctionTokenAST".to_string();

    match first_token.token_type {

        TokenType::Println => {
            let print_node:ASTNode = ast_print::process_print(ast_data, true);
            if ast_data.is_error() {
                return;
            }
            block_node.children.push(print_node);
        },
        _ => {            
            ast_data.error_detail = "unimplemented type in function".to_string();
            ast_data.error_token = first_token;
            ast_data.error_code = AstError::UnimplementedType;
            return;
        },
    }
}
