use crate::convert::go_functions;
use crate::core::enums;
use crate::core::errors;
use crate::core::printing;
use crate::core::structs;
use crate::core::structs::get_default_node;
use enums::ASTNodeType;
use errors::ConvertError;
use structs::ASTNode;
use structs::ConvertData;
use structs::StringBuilder;
use structs::Token;

pub fn convert(ast_nodes:&Vec<ASTNode>, code:&String, convert_error:&mut i32) -> String {
    print!("\t{}Converting{}\t\t", printing::GREY, printing::RESET);

    let mut code_builder = StringBuilder::new();

    let mut convert_data = ConvertData {
        ast_nodes: &ast_nodes,
        code_builder: &mut code_builder,
        error_code: ConvertError::None,
        error_detail: String::new(),
        error_token: structs::empty_token(),
        error_function: String::new(),
        function_return_type: String::new(),
        index_count: 0,
        node_index: 0,
        temp_var_count: 0,
    };

    convert_data.error_function = "convert".to_string();

    let node_count: usize = convert_data.ast_nodes.len();
    if node_count == 0 {
        *convert_error = ConvertError::NoAstNodes;
        return String::new();
    }

    convert_data.code_builder.append_line("package main\n\nimport \"fmt\"\n\n");

    while convert_data.node_index < node_count {

        let previous_index:usize = convert_data.node_index;

        process_global_node(&mut convert_data);
        if convert_data.is_error() {
            print!("\t{}Error{}\n", printing::RED, printing::RESET);
            //debugging::printConvertError(convert_data, code);
            *convert_error = convert_data.error_code;
            return String::new();
        };

        if previous_index == convert_data.node_index {
            convert_data.node_index += 1;
        }
    }
    print!("\t{}Done{}\n", printing::CYAN, printing::RESET);

    let generated_code:StringBuilder = convert_data.code_builder.clone();
    return generated_code.into_string();
}

fn process_global_node(convert_data: &mut ConvertData) {
    
    convert_data.error_function = "processGlobalNode".to_string();

    let node: ASTNode = convert_data.get_node();
    if convert_data.is_error() {
        return;
    }

    unsafe {
        match node.node_type {
            ASTNodeType::FunctionDeclaration => {
                go_functions::process_function_declaration(convert_data, node)
            }
            _ => {
                let token_ptr:*mut Token = node.token;
                let error_token:Token = (*token_ptr).clone();

                convert_data.error_token = error_token;
                convert_data.error_detail = format!("{} not implemented yet", node.node_type);
                convert_data.error_code = ConvertError::UnimplementedNodeType;
                return;
            }
        }
    }
}
