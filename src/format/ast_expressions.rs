use crate::core::debugging;
use crate::core::enums;
use crate::core::errors;
use crate::core::structs;
use crate::core::token_utils;
use crate::format::ast_utils;
use enums::ASTNodeType;
use enums::LoopResult;
use enums::TokenType;
use errors::AstError;
use structs::ASTData;
use structs::ASTNode;
use structs::Token;

fn parse_single_parameter(ast_data: &mut ASTData) -> ASTNode {
    let type_node: ASTNode = create_complex_declarations(ast_data);

    // Now parse the parameter name
    let name_token: Token = ast_data.get_token();
    if ast_data.is_error() {
        return type_node;
    }

    if name_token.token_type != TokenType::Identifier {
        ast_data.error_token = name_token.clone();
        ast_data.error_detail = "Expected identifier for parameter name".to_string();
        return type_node;
    }
    ast_data.token_index += 1;

    let mut parameter_node: ASTNode = structs::get_default_node();

    parameter_node.node_type = ASTNodeType::Parameter;
    let name_token_pointer: *mut Token = structs::create_raw_pointer(name_token);
    parameter_node.token = name_token_pointer;
    let type_node_pointer: *mut ASTNode = structs::create_raw_pointer(type_node);
    parameter_node.left = type_node_pointer;

    return parameter_node;
}

pub fn fill_parameters(ast_data: &mut ASTData) -> ASTNode {
    ast_data.error_function = "fillParameters".to_string();

    let child_list: Vec<ASTNode> = Vec::new();

    let mut parameters_node: ASTNode = structs::get_default_node();
    parameters_node.children = child_list;
    parameters_node.node_type = ASTNodeType::Parameters;

    let mut while_count: usize = 0;

    while ast_data.token_index_in_bounds() {
        if debugging::is_infinite_while(&mut while_count, 1000) {
            ast_data.error_code = AstError::InfiniteWhileLoop;
            return parameters_node;
        }

        let index_before: usize = ast_data.token_index;

        let token: Token = ast_data.get_token();

        if token.token_type == TokenType::RightParenthesis {
            break;
        }
        if token.token_type != TokenType::Comma {
            //print("Add parameter\n", .{});
            let parameter_node: ASTNode = parse_single_parameter(ast_data);
            parameters_node.children.push(parameter_node);
        }

        if index_before == ast_data.token_index {
            ast_data.token_index += 1;
        }
    }

    return parameters_node;
}

fn complex_declaration_inner_loop(
    ast_data: &mut ASTData,
    final_node: &mut ASTNode,
    token: Token,
) -> i32 {
    if token.text == "const" {
        final_node.is_const = true;
        return LoopResult::Continue;
    }

    if token.text == "*" {
        if ast_data.increment_index() == false {
            return LoopResult::Break;
        }

        let mut pointer_node: ASTNode = structs::get_default_node();
        pointer_node.node_type = ASTNodeType::Pointer;
        let token_pointer: *mut Token = structs::create_raw_pointer(token);
        pointer_node.token = token_pointer;
        pointer_node.left = final_node; // Point to the OLD final_node

        // Now reassign the pointer itself
        *final_node = pointer_node; // final_node now points to pointer_node
        return LoopResult::Continue;
    }

    //if token.text == "[" {

    //  ast_data.increment_index();

    //let next_token:Token = ast_data.get_token();
    //if ast_data.is_error() {
    //return LoopResult::Break;
    //}

    //let array_size:usize = 0;

    //if next_token.token_type != TokenType::RightSquareBracket {

    //if next_token.token_type != TokenType::IntegerValue {
    //ast_data.error_detail = "Expected ']' or array size after '[' in array".to_string();
    //ast_data.error_token = next_token;
    //ast_data.error_code = AstError::MissingExpectedType;
    //return LoopResult::Break;
    //}
    //const size = std.fmt.parseInt(usize, next_token.Text, 10) catch {
    //return AstError.Missing_Expected_Type;
    //};
    //array_size = size;
    //const close2:Token = try token_utils_script.getNextToken(ast_data);
    //if (close2.Type != TokenType.RightSquareBracket) {
    //ast_data.setErrorData("Expected ']' or after '[' in array", next_token);
    //return AstError.Missing_Expected_Type;
    //  }
    //}

    //if ast_data.increment_index() {
    // return LoopResult::Break;
    //}

    // var array_node:*ASTNode = try ast_node_utils_script.createDefaultAstNode(allocator);
    //array_node.node_type = ASTNodeType.Array;
    //array_node.token = token;
    //array_node.left = final_node.*;  // Point to the OLD final_node
    //if (array_size != null) {
    //array_node.size = array_size.?;
    //}

    // Now reassign the pointer itself
    //final_node.* = array_node;  // final_node now points to array_node
    //return LoopResult::Continue;
    //}

    return LoopResult::None;
}

fn get_base_type(node: *mut ASTNode) -> *mut ASTNode {
    unsafe {
        if (*node).left.is_null() {
            return node.clone();
        }
        return get_base_type((*node).left);
    }
}

pub fn create_complex_declarations(ast_data: &mut ASTData) -> ASTNode {
    ast_data.error_function = "createComplexDeclarations".to_string();

    let mut final_node: ASTNode = structs::get_default_node();
    let mut while_count: usize = 0;

    let first_token: Token = ast_data.get_token();

    while ast_data.token_index_in_bounds() {
        if debugging::is_infinite_while(&mut while_count, 1000) {
            ast_data.error_code = AstError::InfiniteWhileLoop;
            return final_node;
        }

        let token: Token = ast_data.get_token();
        if ast_data.is_error() {
            return final_node;
        }

        // Handle inner loop declarations like array size brackets
        let loop_result: i32 =
            complex_declaration_inner_loop(ast_data, &mut final_node, token.clone());
        if loop_result == LoopResult::Continue {
            continue;
        }

        // If the token is a base type (e.g., "i32"), attach it at the innermost level
        if token_utils::is_type_token(token.clone()) {
            if ast_data.increment_index() == false {
                return final_node;
            }

            let mut type_node: ASTNode = structs::get_default_node();
            type_node.node_type = ASTNodeType::VarType;
            type_node.token = Box::into_raw(Box::new(token));

            if final_node.node_type == ASTNodeType::Invalid {
                final_node = type_node;
                break;
            }

            // Traverse to the leftmost node and attach the type
            let mut innermost: *mut ASTNode = &mut final_node as *mut ASTNode;
            let mut inner_while_count: usize = 0;

            unsafe {
                while !(*innermost).left.is_null() {
                    if debugging::is_infinite_while(&mut inner_while_count, 1000) {
                        ast_data.error_code = AstError::InfiniteWhileLoop;
                        return final_node;
                    }
                    innermost = (*innermost).left;
                }
                (*innermost).left = Box::into_raw(Box::new(type_node));
            }
            break;
        }

        break;
    }

    if final_node.node_type == ASTNodeType::Invalid {
        ast_data.error_token = first_token;
        ast_data.error_detail = "final node is null in createComplexDeclarations".to_string();
        ast_data.error_code = AstError::NullType;
        return final_node;
    }

    return final_node;
}

pub fn parsePrimaryAny(ast_data: &mut ASTData) -> ASTNode {
    ast_data.error_function = "parsePrimaryAny".to_string();

    if ast_data.token_index_in_bounds() == false {
        ast_data.error_code = AstError::UnexpectedEndOfFile;
        return structs::get_default_node();
    }

    let token: Token = ast_data.get_token();
    let mut node: ASTNode = structs::get_default_node();

    match token.token_type {
        TokenType::False | TokenType::True => {
            node.node_type = ASTNodeType::BoolLiteral;
            node.token = structs::create_raw_pointer(token);
        }
        TokenType::IntegerValue => {
            node.node_type = ASTNodeType::StringLiteral;
            node.token = structs::create_raw_pointer(token);
        }
        //TokenType.Identifier => try parseProcessIdentifier(allocator, ast_data, node, token),
        TokenType::StringValue => {
            node.node_type = ASTNodeType::StringLiteral;
            node.token = structs::create_raw_pointer(token);
        }
        TokenType::CharValue => {
            node.node_type = ASTNodeType::CharLiteral;
            node.token = structs::create_raw_pointer(token);
        }
        //TokenType.LeftParenthesis => return ProcessParsePrimaryLeftParenthesis(ast_data, node_type, allocator),
        TokenType::RightParenthesis => return node,
        //TokenType.And => return ProcessReference(ast_data, token, allocator),
        //TokenType.Minus => try parseMinus(allocator, ast_data, node, token),
        _ => {
            ast_data.error_detail = "Unexpected type in expression, {token.Type}".to_string();
            ast_data.error_token = token;
            ast_data.error_code = AstError::UnexpectedType;
            return node;
        }
    }

    ast_data.token_index += 1;
    return node;
}

pub fn parse_binary_expression_any(ast_data:&mut ASTData, min_prec:usize, node_type:i32) -> ASTNode {

    ast_data.error_function = "parseBinaryExprAny".to_string();
    let mut left: ASTNode = parsePrimaryAny(ast_data);

    let mut white_count: usize = 0;
    let max_loops: usize = 1000;

    while ast_data.token_index_in_bounds() {
        
        if debugging::is_infinite_while(&mut white_count, max_loops) {
            return left;
        }

        let operator_token: Token = ast_data.get_token();

        if token_utils::is_binary_operator_bool(operator_token.token_type) == false {
            break;
        }

        let precedence: usize = token_utils::get_precedence_bool(operator_token.token_type);

        if precedence < min_prec {
            break;
        }

        ast_data.token_index += 1; // move past operator
        let right: ASTNode = parse_binary_expression_any(ast_data, precedence + 1, node_type);
        if ast_data.is_error() {
            return left;
        }

        if right.node_type == ASTNodeType::Invalid {
            ast_data.error_detail = "Missing value after equation symbol".to_string();
            ast_data.error_token = operator_token;
            ast_data.error_code = AstError::UnexpectedType;
            return left;
        }

        let mut new_node: ASTNode = structs::get_default_node();

        new_node.node_type = node_type;
        new_node.token = structs::create_raw_pointer(operator_token);
        new_node.left = structs::create_raw_pointer(left);
        new_node.right = structs::create_raw_pointer(right);

        left = new_node;
    }
    return left;
}
