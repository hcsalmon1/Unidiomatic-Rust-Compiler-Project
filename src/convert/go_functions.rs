

use crate::core::errors;
use crate::core::structs;
use crate::convert::go_body;
use crate::convert::go_utils; 
use structs::ASTNode;
use structs::Token;
use structs::ConvertData;
use errors::ConvertError;


pub fn process_function_declaration(convert_data:&mut ConvertData, node:ASTNode) {

    convert_data.error_function = "process_function_declaration".to_string();

    //Node_type: ASTNode_FunctionDeclaration,
    //Token:     third_token, - function name
    //Left:      &type_node, - return type node (i32, etc.)
    //Middle:    nil, - parameters
    //Right:     function_body_node, - function body

    write_function_name_and_parameters(convert_data, &node);

    if node.right.is_null() {
        convert_data.error_detail = "node.Right is null".to_string();
        convert_data.error_code = ConvertError::NodeIsNull;
        return;
    }

    convert_data.increment_index_count();
    unsafe  {
        go_body::process_body(convert_data, (*node.right).clone());
    }
    convert_data.decrement_index_count();

    convert_data.code_builder.append("\n\r}\n\n");
}

fn write_function_name_and_parameters(convert_data:&mut ConvertData, node:&ASTNode) {

    unsafe {
    convert_data.error_function = "writeFunctionNameAndParameters".to_string();

    if node.left.is_null() {
        convert_data.error_detail = "node.Left is null".to_string();
        convert_data.error_code = ConvertError::NodeIsNull;
        return;
    }

    let function_name_ptr:*mut Token = node.token;
    if function_name_ptr.is_null() {
        convert_data.error_code = ConvertError::NodeIsNull;
        return;
    }
    let function_name_token:Token = (*function_name_ptr).clone();

    //write declaration
    let return_type_text:String = convert_data.print_type(node.left).text;

    convert_data.code_builder.append_fmt(format!("func {}(", function_name_token.text));

    if node.middle.is_null() == false {
        print_parameters(convert_data, node);
    }

    if return_type_text == "void" {
        convert_data.code_builder.append(") {\n");
        return;
    }

    convert_data.code_builder.append_fmt(format!(") {} ", return_type_text));
    convert_data.code_builder.append("{\n");
    }
}

fn print_parameters(convert_data:&mut ConvertData, node:&ASTNode) {

    unsafe {
    if node.middle.is_null() {
        convert_data.error_detail = "Internal error: node.middle is null in printParameters".to_string();
        convert_data.error_code = ConvertError::NodeIsNull;
        return;
    }

    let middle_ptr:*mut ASTNode = node.middle;
    let middle_node:ASTNode = (*middle_ptr).clone();

    let child_count:usize = middle_node.children.len();

    let child_list:Vec<ASTNode> = middle_node.children;

    if child_count == 0 {
        return;
    }
    for i in 0..child_count {

        let child:&ASTNode = &child_list[i];

        let parameter_token_ptr:*mut Token = child.token;
        let parameter_token:Token = (*parameter_token_ptr).clone();
        let parameter_name:String = parameter_token.text;

        let type_node_ptr:*mut ASTNode = child.left;
        if type_node_ptr.is_null() {
            convert_data.error_detail = "Internal error: type_node is null in printParameters".to_string();
            return;
        }

        //Recursively loop for the last node and expect it be to a type
        let mut base_type_node_ptr:*mut ASTNode = type_node_ptr;
        while (*base_type_node_ptr).left.is_null() == false {
            base_type_node_ptr = (*base_type_node_ptr).left;
        }

        if base_type_node_ptr.is_null() {
            convert_data.error_detail = "Internal error: base_type_node is null in printParameters".to_string();
            convert_data.error_code = ConvertError::NodeIsNull;
            return;
        }

        let base_type_node:ASTNode = (*base_type_node_ptr).clone();
        let base_type_token_ptr:*mut Token = base_type_node.token;
        let base_type_token:Token = (*base_type_token_ptr).clone();

        let var_type:&str = go_utils::convert_type_to_go_type(base_type_token.token_type);

        convert_data.code_builder.append_fmt(format!("{} {}", parameter_name, var_type));


        if i < child_count - 1 {
            convert_data.code_builder.append(", ");
        }
    }
}
}