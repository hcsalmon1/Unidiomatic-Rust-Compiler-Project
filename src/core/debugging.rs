use crate::core::enums;
use crate::core::printing;
use crate::core::structs;
use crate::core::structs::ASTNode;
use crate::core::structs::Token;
use std::str::Lines;
use structs::ASTData;

pub fn print_tokens(tokens:&Vec<Token>) {
    println!("{}Printing tokens:{}", printing::GREY, printing::RESET);

    let token_count:usize = tokens.len();

    if token_count == 0 {
        println!("\tNo tokens");
        return;
    }
    for i in 0..token_count {
        let token:&Token = &tokens[i];
        println!(
            "\ttoken: {}'{}'{} type: {}{}{}",
            printing::ORANGE,
            token.text,
            printing::RESET,
            printing::CREAM,
            enums::token_type_to_string(token.token_type),
            printing::RESET
        );
    }
}

pub fn is_infinite_while(while_count: &mut usize, while_cap: usize) -> bool {
    if *while_count >= while_cap {
        return true;
    }
    *while_count += 1;
    return false;
}

pub fn print_ast_nodes(ast_nodes: &Vec<ASTNode>) {
    print!(
        "\n{}Printing Ast Nodes:{}\n",
        printing::GREY,
        printing::RESET
    );

    let node_count: usize = ast_nodes.len();

    if node_count == 0 {
        print!("{}\tNo nodes{}\n", printing::RED, printing::RESET);
        return;
    }

    for i in 0..node_count {
        let node:&ASTNode = &ast_nodes[i];
        print_ast_node_as_ref(node, 3, "base");
    }
}

use core::slice::Iter;

pub fn print_ast_node_as_ref(node: &ASTNode, indent: usize, ast_type_text: &str) {
    unsafe {
        // Create padding
        let mut padding_builder: Vec<char> = Vec::new();

        for i in 0..indent {
            add_spacing(indent, &mut padding_builder, i);
        }

        let padding_length: usize = padding_builder.len();

        if padding_length > 0 {

            let padding_iterator:Iter<'_, char> = padding_builder[0..padding_length].iter();
            let padding:String = padding_iterator.collect();

            // Print node info
            if padding.len() > 0 {
                print!("{}", padding);
            }
        }

        print!(
            "{}{}{} ",
            printing::CREAM,
            enums::astnodetype_to_string(node.node_type),
            printing::RESET
        );

        // Print token info
        if node.token.is_null() == false {
            let token: *mut Token = node.token;

            println!(
                "{}'{}'{} - {}",
                printing::CYAN,
                (*token).text,
                printing::RESET,
                ast_type_text,
            );
        } else {
            println!("NA - {}", ast_type_text);
        }

        // Recursively print children with increased indent
        print_ast_node_ptr(node.left, indent + 1, "left");
        print_ast_node_ptr(node.middle, indent + 1, "middle");
        print_ast_node_ptr(node.right, indent + 1, "right");

        let child_count: usize = node.children.len();

        for i in 0..child_count {
            let child: ASTNode = node.children[i].clone();
            print_ast_node_as_ref(&child, indent + 1, "child");
        }
    }
}

pub fn print_ast_node_ptr(node: *mut ASTNode, indent: usize, ast_type_text: &str) {
    unsafe {
        if node.is_null() {
            return;
        }

        // Create padding
        let mut padding_builder: Vec<char> = Vec::new();

        for i in 0..indent {
            add_spacing(indent, &mut padding_builder, i);
        }

        let padding_length: usize = padding_builder.len();
        if padding_length > 0 {
            let padding: String = padding_builder[0..padding_length].iter().collect();
            if padding.len() > 0 {
                print!("{}", padding);
            }
        }

        let node_ref = &*node; // Dereference once, store reference

        print!(
            "{}{}{} ",
            printing::CREAM,
            enums::astnodetype_to_string(node_ref.node_type),
            printing::RESET
        );

        // Print token info
        if node_ref.token.is_null() == false {
            let token_ref = &*node_ref.token;
            println!(
                "{}'{}'{} - {}",
                printing::CYAN,
                token_ref.text,
                printing::RESET,
                ast_type_text,
            );
        } else {
            println!("NA - {}", ast_type_text);
        }

        // Recursively print children
        print_ast_node_ptr(node_ref.left, indent + 1, "left");
        print_ast_node_ptr(node_ref.middle, indent + 1, "middle");
        print_ast_node_ptr(node_ref.right, indent + 1, "right");

        for i in 0..node_ref.children.len() {
            let child = node_ref.children[i].clone(); // Just copy the pointer
            print_ast_node_as_ref(&child, indent + 1, "child");
        }
    }
}

fn add_spacing(indent: usize, padding: &mut Vec<char>, i: usize) {
    if i + 1 == indent {
        padding.push('|');
        padding.push('-');
        return;
    }
    if i == 2 {
        padding.push('|');
        padding.push(' ');
        return;
    }
    if i == 3 {
        padding.push('|');
        padding.push(' ');
        return;
    }

    padding.push(' ');
    padding.push(' ');
}

pub fn print_ast_error(ast_data: &ASTData, code: &String) {
    let error_token: Token = ast_data.error_token.clone();

    let line_number: usize = error_token.line_number;
    let char_number: usize = error_token.char_number;
    let error_detail: String = ast_data.error_detail.clone();

    let line_iterator: Lines<'_> = code.lines();
    let code_lines: Vec<&str> = line_iterator.collect();

    print!(
        "\t{}Error on line {}, {}: {}{}\n",
        printing::CREAM,
        line_number + 1,
        char_number,
        error_detail,
        printing::RESET
    );

    print_code_lines(line_number, char_number, &code_lines);

    println!("Error token: {}", error_token.text);

    println!("Error function: {}", ast_data.error_function);
}

fn print_code_lines(line_number: usize, char_number: usize, code_lines: &Vec<&str>) {
    let line_count: usize = code_lines.len();
    let mut previous_line: &str = "...";

    let mut previous_index_in_range: bool = false;

    if line_number > 0 {
        previous_index_in_range = line_number - 1 < line_count;
    }

    if previous_index_in_range {
        previous_line = code_lines[line_number - 1];
    }

    let mut code_line: &str = "...";

    let index_in_range: bool = line_number < line_count;

    if index_in_range {
        code_line = code_lines[line_number];
    }

    print!(
        "\tline {}: {}\n\tline {}: {}\n\t\t{}",
        line_number,
        previous_line,
        line_number + 1,
        code_line,
        printing::GREEN
    );
    let mut i: usize = 0;
    while i < char_number {
        print!("~");
        i += 1;
    }
    print!("^{}\n\n", printing::RESET);
}
