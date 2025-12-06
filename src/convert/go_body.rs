use std::fmt::format;

use crate::convert::go_body;
use crate::convert::go_print;
use crate::convert::go_utils;
use crate::core::debugging;
use crate::core::enums;
use crate::core::errors;
use crate::core::printing;
use crate::core::structs;
use crate::format::ast_expressions;
use crate::format::ast_functions;
use crate::format::ast_utils;
use enums::ASTNodeType;
use enums::TokenType;
use errors::AstError;
use errors::ConvertError;
use structs::ASTData;
use structs::ASTNode;
use structs::ConvertData;
use structs::StringBuilder;
use structs::Token;

const NEW_LINE: bool = true;
const TABS: bool = true;

pub fn process_body(convert_data: &mut ConvertData, node: ASTNode) {
    
    convert_data.error_function = "processBody".to_string();

    //NodeType Type
    //Children body nodes

    let child_count: usize = node.children.len();
    if child_count == 0 {
        return;
    }
    for i in 0..child_count {
        let child:ASTNode = node.children[i].clone();
        process_function_body_node(convert_data, child, NEW_LINE, TABS);
    }
}

fn process_function_body_node(convert_data:&mut ConvertData, node:ASTNode, add_new_line:bool, add_tabs:bool) {
    unsafe {
        convert_data.error_function = "processFunctionBodyNode".to_string();
        let node_type: i32 = node.node_type;

        if node_type == ASTNodeType::Invalid {
            convert_data.error_token = (*node.token).clone();
            convert_data.error_code = ConvertError::InvalidNodeType;
            return;
        }

        match node_type {
            ASTNodeType::Println => go_print::process_print(convert_data, node, true),
            _ => {
                convert_data.error_token = (*node.token).clone();
                convert_data.error_detail = format!(
                    "{} not implemented yet",
                    enums::astnodetype_to_string(node.node_type)
                );
                convert_data.error_code = ConvertError::UnimplementedNodeType;
                return;
            }
        }
    }
}
