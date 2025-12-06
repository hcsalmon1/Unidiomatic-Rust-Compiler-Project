use crate::core::enums::ASTNodeType;
use crate::core::enums::TokenType;
use crate::core::errors::AstError;
use crate::core::errors::ConvertError;

pub struct ParseData<'lifetimes_suck> {
    pub token_list: Vec<Token>,
    pub last_token: i32,
    pub character_index: usize,
    pub code: &'lifetimes_suck [u8],
    pub line_count: usize, //for token position
    pub char_count: usize,
    pub was_comment: bool,
}

pub struct Token {
    pub text: String,
    pub token_type: i32,
    pub line_number: usize,
    pub char_number: usize,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            text: self.text.clone(),
            token_type: self.token_type,
            line_number: self.line_number,
            char_number: self.char_number,
        }
    }
}

pub fn empty_token() -> Token {
    return Token {
        text: String::new(),
        token_type: TokenType::Na,
        line_number: 0,
        char_number: 0,
    };
}

pub struct ASTData<'lifetimes_suck> {
    pub ast_nodes: &'lifetimes_suck mut Vec<ASTNode>,
    pub token_index: usize,
    pub token_list: &'lifetimes_suck Vec<Token>,
    pub error_detail: String,
    pub error_token: Token,
    pub error_function: String,
    pub error_code: i32,
}

impl<'lifetimes_suck> ASTData<'lifetimes_suck> {
    pub fn get_token(&mut self) -> Token {
        if self.token_index >= self.token_list.len() {
            self.error_code = AstError::IndexOutOfRange;
            return empty_token();
        }
        return self.token_list[self.token_index].clone();
    }

    pub fn token_index_in_bounds(&self) -> bool {
        if self.token_index >= self.token_list.len() {
            return false;
        }
        return true;
    }

    pub fn is_error(&self) -> bool {
        return self.error_code != 0;
    }
    pub fn append_node(&mut self, node: ASTNode) {
        self.ast_nodes.push(node);
    }
    pub fn increment_index(&mut self) -> bool {
        if self.token_index + 1 >= self.token_list.len() {
            self.error_code = AstError::UnexpectedEndOfFile;
            return false;
        }
        self.token_index += 1;
        return true;
    }
    pub fn expect_type(&mut self, expected_type: i32, error_message: &str) -> bool {
        let token: Token = self.get_token();
        if self.is_error() {
            return false;
        }
        if token.token_type != expected_type {
            self.error_detail = error_message.to_string();
            self.error_code = AstError::MissingExpectedType;
            return false;
        }
        return true;
    }
}

pub fn create_raw_pointer<T>(variable: T) -> *mut T {
    return Box::into_raw(Box::new(variable));
}

pub struct ASTNode {
    pub node_type: i32,
    pub token: *mut Token,  // Just store it directly, use a default/dummy value
    pub left: *mut ASTNode, // null pointer = no left node
    pub middle: *mut ASTNode,
    pub right: *mut ASTNode,
    pub children: Vec<ASTNode>,
    pub is_const: bool,
    pub size: usize,
}

impl Clone for ASTNode {
    fn clone(&self) -> Self {
        ASTNode {
            node_type: self.node_type,
            token: self.token.clone(),
            left: self.left.clone(),
            middle: self.middle.clone(),
            right: self.right.clone(),
            children: self.children.clone(),
            is_const: self.is_const,
            size: self.size,
        }
    }
}

pub fn get_default_node() -> ASTNode {
    return ASTNode {
        node_type: ASTNodeType::Invalid,
        token: std::ptr::null_mut(),
        left: std::ptr::null_mut(),
        middle: std::ptr::null_mut(),
        right: std::ptr::null_mut(),
        children: Vec::new(),
        is_const: false,
        size: 0,
    };
}

pub struct StringBuilder {
    buffer: Vec<u8>,
}

impl StringBuilder {
    /// Create a new StringBuilder
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Create a new StringBuilder with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    /// Append a string
    pub fn append(&mut self, s: &str) {
        self.buffer.extend_from_slice(s.as_bytes());
    }

    /// Append with formatting
    pub fn append_fmt(&mut self, s: String) {
        self.buffer.extend_from_slice(s.as_bytes());
    }

    /// Append a line (adds newline)
    pub fn append_line(&mut self, s: &str) {
        self.buffer.extend_from_slice(s.as_bytes());
        self.buffer.push(b'\n');
    }

    /// Append formatted line
    pub fn append_line_fmt(&mut self, s: String) {
        self.buffer.extend_from_slice(s.as_bytes());
        self.buffer.push(b'\n');
    }

    /// Get the string as a slice
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.buffer) }
    }

    /// Convert to owned String (consumes self)
    pub fn into_string(self) -> String {
        unsafe {
            return String::from_utf8_unchecked(self.buffer);
        }
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
impl Clone for StringBuilder {
    fn clone(&self) -> Self {
        StringBuilder {
            buffer: self.buffer.clone(),
        }
    }
}

pub struct ConvertData<'lifetimes_suck> {
    pub ast_nodes:&'lifetimes_suck Vec<ASTNode>,
    pub node_index:usize,
    pub error_code:i32,
    pub error_detail:String,
    pub error_token:Token,
    pub error_function:String,
    pub code_builder:&'lifetimes_suck mut StringBuilder,
    pub temp_var_count:usize,
    pub function_return_type:String,
    pub index_count:usize,
}

impl<'lifetimes_suck> ConvertData<'lifetimes_suck> {

    pub fn is_error(&self) -> bool {
        return self.error_code != ConvertError::None;
    }

    pub fn get_node(&mut self) -> ASTNode {
        if self.node_index >= self.ast_nodes.len() {
            self.error_code = ConvertError::InvalidNodeType;
            return get_default_node();
        }
        return self.ast_nodes[self.node_index].clone();
    }

    pub fn increment_index_count(&mut self) {
        self.index_count += 1;
    }

    pub fn decrement_index_count(&mut self) {
        if self.index_count == 0 {
            return;
        }
        self.index_count -= 1;
    }

    pub fn print_type(&self, node_ptr:*mut ASTNode) -> Token {
    unsafe {
        let node:ASTNode = (*node_ptr).clone();
        let token_ptr:*mut Token = node.token;
        return (*token_ptr).clone(); 
    }
    }
}
