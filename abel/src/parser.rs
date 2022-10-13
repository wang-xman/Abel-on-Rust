use crate::error::{ErrorKind, Error, ParserError};
use crate::delimiter;
use crate::token::Token;

pub struct Parser<TokenT, ContainerT, SchemeT> {
    pub token_vector: Vec<TokenT>,
    pub current_line: i32,
    pub current_column: i32,
    // latest_symbol: String,
    pub current_literal: String,
    pub current_level: i32,
    pub deepest_level: i32,
    pub current_container_type: Vec<ContainerT>,
    pub current_iter_index: Vec<i32>,
    pub parent_key: Vec<String>,
    pub keys_per_level: Vec<Vec<TokenT>>,
    pub latest_syntactic_operator: String,
    pub is_escaping: bool,
    pub is_delimited_string_open: bool,
    pub current_literal_scheme: SchemeT,
    pub bracket_match: delimiter::DelimiterMatch,
    pub parser_type: String,
}

impl<TokenT, ContainerT, SchemeT> Parser<TokenT, ContainerT, SchemeT>
where
    TokenT: Token,
    ContainerT: Copy
{
    pub fn size(&self) -> usize {
        self.token_vector.len()
    }

    pub fn push(&mut self, token: TokenT) {
        self.token_vector.push(token);
    }

    pub fn reset_current_literal(&mut self) {
        self.current_literal.clear();
    }

    pub fn get_token_vector(&self) -> &Vec<TokenT> {
        &self.token_vector
    }
}

// Implement getters, setters and operational methods
impl<TokenT, ContainerT, SchemeT> Parser<TokenT, ContainerT, SchemeT>
where
    TokenT: Token,
    ContainerT: Copy
{
    pub fn get_current_container_type(&self) -> ContainerT {
        self.current_container_type[self.current_level as usize]
    }

    pub fn get_parent_container_type(&self) -> ContainerT {
        self.current_container_type[(self.current_level - 1) as usize]
    }

    pub fn get_root_container_type(&self) -> ContainerT {
        self.current_container_type[0]
    }

    pub fn enter_deeper_level(&mut self) {
        self.current_level += 1;
    }

    pub fn enter_higher_level(&mut self) {
        self.current_level -= 1;
    }

    pub fn is_first_noncomment_character(&self) -> bool {
        self.current_literal == "" && self.token_vector.len() == 0
    }

    pub fn illegal_first_noncomment_character(&self, first_char: &str)
    -> Result<(), ParserError>
    {
        let mut error = ParserError::new(
            &format!("Symbol {} cannot be the first character.", first_char),
            ErrorKind::SyntaxError);
        error.set_line(self.current_line);
        Err(error)
    }
}