#![allow(non_upper_case_globals)]

pub struct ParseError;
impl ParseError {
    pub const None:i32 = 0;
    pub const Code_Length_Is_Zero: i32 = 1;
    pub const Unterminated_String: i32 = 2;
    pub const Unexpected_Value: i32 = 3;
    pub const Unterminated_Char: i32 = 4;
}

pub fn parse_error_to_string(error: i32) -> &'static str {
    match error {
        ParseError::Code_Length_Is_Zero => "Code_Length_Is_Zero",
        ParseError::Unterminated_String => "Unterminated_String",
        ParseError::Unexpected_Value => "Unexpected_Value",
        ParseError::Unterminated_Char => "Unterminated_Char",
        _ => "Unknown",
    }
}

pub struct AstError;
impl AstError {
    pub const None: i32 = 0;
    pub const InfiniteWhileLoop: i32 = 1;
    pub const IndexOutOfRange: i32 = 2;
    pub const InvalidDeclaration: i32 = 3;
    pub const UnexpectedType: i32 = 4;
    pub const UnimplementedType: i32 = 5;
    pub const MissingExpectedType: i32 = 6;
    pub const UnexpectedEndOfFile: i32 = 7;
    pub const NullType: i32 = 8;
    pub const OutOfMemory: i32 = 9;
}

pub fn ast_error_to_string(error: i32) -> &'static str {
    match error {
        AstError::InfiniteWhileLoop => "Infinite_While_Loop",
        AstError::IndexOutOfRange => "Index_Out_Of_Range",
        AstError::InvalidDeclaration => "Invalid_Declaration",
        AstError::UnexpectedType => "Unexpected_Type",
        AstError::UnimplementedType => "Unimplemented_Type",
        AstError::MissingExpectedType => "Missing_Expected_Type",
        AstError::UnexpectedEndOfFile => "Unexpected_End_Of_File",
        AstError::NullType => "Null_Type",
        AstError::OutOfMemory => "Out_Of_Memory",
        _ => "Unknown",
    }
}

pub struct ConvertError;
impl ConvertError {
    pub const None:i32 = 0;
    pub const NodeIsNull: i32 = 1;
    pub const NoAstNodes: i32 = 2;
    pub const OutOfMemory: i32 = 3;
    pub const UnimplementedNodeType: i32 = 4;
    pub const InvalidReturnType: i32 = 5;
    pub const InvalidNodeType: i32 = 6;
    pub const NodeIndexOutOfRange: i32 = 7;
}

pub fn convert_error_to_string(error: i32) -> &'static str {
    match error {
        ConvertError::NodeIsNull => "Node_Is_Null",
        ConvertError::NoAstNodes => "No_AST_Nodes",
        ConvertError::OutOfMemory => "Out_Of_Memory",
        ConvertError::UnimplementedNodeType => "Unimplemented_Node_Type",
        ConvertError::InvalidReturnType => "Invalid_Return_Type",
        ConvertError::InvalidNodeType => "Invalid_Node_Type",
        ConvertError::NodeIndexOutOfRange => "NodeIndexOutOfRange",
        _ => "Unknown",
    }
}

pub struct SemanticError;
impl SemanticError {
    pub const None:i32 = 0;
    pub const FunctionRedefinition: i32 = 1;
    pub const VariableRedefinition: i32 = 2;
    pub const OutOfMemory: i32 = 3;
}

pub fn semantic_error_to_string(error: i32) -> &'static str {
    match error {
        SemanticError::FunctionRedefinition => "Function_Redefinition",
        SemanticError::VariableRedefinition => "Variable_Redefinition",
        SemanticError::OutOfMemory => "Out_Of_Memory",
        _ => "Unknown",
    }
}