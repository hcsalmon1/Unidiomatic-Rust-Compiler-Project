use crate::core::enums::ASTNodeType;
use crate::core::structs;
use structs::ASTNode;
use structs::ConvertData;
use structs::Token;

pub fn process_print(convert_data:&mut ConvertData, node:ASTNode, new_line:bool) {
    //Children = nodes to print
    //node type - print or println
    //token - print token for debugging

    convert_data.code_builder.append("\t");

    let child_count:usize = node.children.len();

    if child_count == 0 {
        if new_line == true {
            convert_data.code_builder.append("fmt.println()\n\t");
            convert_data.code_builder.append("\n");
        }
        return;
    }

    if new_line {
        convert_data.code_builder.append("fmt.Println(");
    } else {
        convert_data.code_builder.append("fmt.Print(");
    }

    for i in 0..child_count {
        unsafe {
            let child:&ASTNode = &node.children[i];
            let token_ptr:*mut Token = child.token;
            let token:Token = (*token_ptr).clone();

            if i != 0 {
                convert_data.code_builder.append(", ");
            }
            if child.node_type == ASTNodeType::StringLiteral {
                convert_data.code_builder.append("\"");
            }
            convert_data.code_builder.append(token.text.as_str());
            if child.node_type == ASTNodeType::StringLiteral {
                convert_data.code_builder.append("\"");
            }
        }
    }
    convert_data.code_builder.append(")");
}
