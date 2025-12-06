use crate::core::enums;
use crate::core::errors;
use crate::core::structs;
use crate::format::ast_expressions;
use enums::TokenType;
use errors::AstError;
use structs::ASTData;
use structs::ASTNode;
use structs::Token;

pub fn fill_node_in_brackets(ast_data:&mut ASTData, output_node:&mut ASTNode, node_type:i32) {

    ast_data.error_function = "fillNodeInBrackets".to_string();
    let mut count: usize = 0;

    loop {
        count += 1;

        if count > 500 {
            ast_data.error_code = AstError::InfiniteWhileLoop;
            return;
        }

        let token: Token = ast_data.get_token();
        if ast_data.is_error() {
            return;
        }

        if token.token_type == TokenType::RightParenthesis {
            break;
        }

        if token.token_type == TokenType::Comma {
            ast_data.token_index += 1;
            continue;
        }

        let value_node: ASTNode =
            ast_expressions::parse_binary_expression_any(ast_data, 0, node_type);

        output_node.children.push(value_node);
    }
}
