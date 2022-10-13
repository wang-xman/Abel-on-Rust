use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::error::{ErrorKind, Error, ParserError};
use crate::typefy::NamedType;
use crate::symbol;
use crate::util;
use crate::delimiter;
use crate::converter;
use crate::parser::Parser;
use crate::json_token::{JsonTokenType, JsonToken, JsonContainerType,
    JsonTerminalType, LiteralScheme, tokenize, tokenize_key,
    tokenize_iter_key, tokenize_terminal, get_token_type_by_symbol,
    get_container_type_by_symbol};

pub type JsonParser = Parser<JsonToken, JsonContainerType, LiteralScheme>;

impl JsonParser {
    // TODO
    fn set_root_container_type(&mut self, container_type: JsonContainerType)
    -> Result<(), ParserError>
    {
        if container_type != JsonContainerType::None {
            if self.current_container_type[0] == JsonContainerType::List
                    || self.current_container_type[0] == JsonContainerType::Dict
            {
                let msg = format!("Manually override the root container which \
                    has already been set to {}.",
                    self.current_container_type[0].type_name());
                Err(ParserError::new(&msg, ErrorKind::SyntaxError))
            } else {
                self.current_container_type[0] = container_type;
                Ok(())
            }
        } else {
            let msg = format!("Unknown type for root container. \
                Root container type can be either List or Dict.");
            Err(ParserError::new(&msg, ErrorKind::SyntaxError))
        }
    }

    fn is_current_container_iterable(&self) -> bool {
        self.get_current_container_type() == JsonContainerType::List
    }

    /// Terminal types for standard JSON, used by `json` type parser
    pub fn get_terminal_type(&self, literal: &str, scheme: LiteralScheme)
    -> Result<JsonTerminalType, ParserError>
    {
        if scheme == LiteralScheme::Liberal {
            if converter::is_null(literal) {
                Ok(JsonTerminalType::Null)
            } else if converter::is_bool(literal) {
                Ok(JsonTerminalType::Bool)
            } else if converter::is_double(literal) {
                Ok(JsonTerminalType::Double)
            } else {
                let msg = format!("Intended data type of unquoted string '{}' \
                                  cannot be recognised.", literal);
                let mut error = ParserError::new(&msg, ErrorKind::SyntaxError);
                error.set_line(self.current_line);
                Err(error)
            }
        } else if scheme == LiteralScheme::None {
            // this scenario must not occur.
            let msg = format!("Collection scheme of string '{}' is \
                              not set.", literal);
            let mut error = ParserError::new(&msg, ErrorKind::SyntaxError);
            error.set_line(self.current_line);
            Err(error)
        } else { // delimited strings are always Text
            Ok(JsonTerminalType::Text)
        }
    }

    /// Terminal types for JSON+ format, used by `json_plus` type
    pub fn get_terminal_plus_type(&self, literal: &str, scheme: LiteralScheme)
    -> Result<JsonTerminalType, ParserError>
    {
        if scheme == LiteralScheme::Liberal {
            if converter::is_null(literal) {
                Ok(JsonTerminalType::Null)
            } else if converter::is_bool(literal) {
                Ok(JsonTerminalType::Bool)
            } else if converter::is_integer(literal) {
                Ok(JsonTerminalType::Integer)
            } else if converter::is_double(literal) {
                Ok(JsonTerminalType::Double)
            } else if converter::is_complex(literal) {
                Ok(JsonTerminalType::Complex)
            } else if converter::is_binary(literal) {
                Ok(JsonTerminalType::Binary)
            } else if converter::is_bitstring(literal) {
                Ok(JsonTerminalType::Bitstring)
            } else {
                let msg = format!("Intended data type of unquoted string '{}' \
                                  cannot be recognised for JSON+.", literal);
                let mut error = ParserError::new(&msg, ErrorKind::SyntaxError);
                error.set_line(self.current_line);
                Err(error)
            }
        } else if scheme == LiteralScheme::None {
            // this scenario must not occur.
            let msg = format!("Collection scheme of string '{}' is \
                              not set.", literal);
            let mut error = ParserError::new(&msg, ErrorKind::SyntaxError);
            error.set_line(self.current_line);
            Err(error)
        } else { // delimited strings are always Text
            Ok(JsonTerminalType::Text)
        }
    }
}

impl JsonParser {
    pub fn new() -> Self { // default parser is JSON type
        JsonParser {
            token_vector: Vec::<JsonToken>::new(),
            current_line: 0,
            current_column: 0,
            //latest_symbol: String::new(),
            current_literal: String::new(),
            current_level: 0,
            deepest_level: 0,
            current_container_type: vec![JsonContainerType::None],
            current_iter_index: vec![0],
            parent_key: vec![String::from("ROOT_KEY_")],
            keys_per_level: vec![vec![]],
            latest_syntactic_operator: String::new(),
            is_escaping: false,
            is_delimited_string_open: false,
            current_literal_scheme: LiteralScheme::None,
            bracket_match: delimiter::DelimiterMatch::new(),
            parser_type: String::from("json")
        }
    }

    pub fn new_plus() -> Self { // for JSON+ format
        JsonParser {
            token_vector: Vec::<JsonToken>::new(),
            current_line: 0,
            current_column: 0,
            //latest_symbol: String::new(),
            current_literal: String::new(),
            current_level: 0,
            deepest_level: 0,
            current_container_type: vec![JsonContainerType::None],
            current_iter_index: vec![0],
            parent_key: vec![String::from("ROOT_KEY_")],
            keys_per_level: vec![vec![]],
            latest_syntactic_operator: String::new(),
            is_escaping: false,
            is_delimited_string_open: false,
            current_literal_scheme: LiteralScheme::None,
            bracket_match: delimiter::DelimiterMatch::new(),
            parser_type: String::from("json_plus")
        }
    }

    fn report_duplicate_key(&mut self, token: JsonToken)
    -> Result<(), ParserError>
    {
        let mut is_duplicate = false;
        let mut err_msg = String::new();
        if self.keys_per_level.len() <= self.current_level as usize {
            self.keys_per_level.push(vec![token]);
            return Ok(());
        } else {
            for token_exist in &self.keys_per_level[self.current_level as usize] {
                if token_exist.parent_key == token.parent_key
                        && token_exist.literal == token.literal {
                    // caught a duplicate key
                    is_duplicate = true;
                    err_msg = format!("Key '{}' is a duplicate.", token.literal);
                }
            }
            if !is_duplicate {
                self.keys_per_level[self.current_level as usize].push(token);
                return Ok(());
            } else {
                let mut error = ParserError::new(&err_msg, ErrorKind::DuplicateKey);
                error.set_line(self.current_line);
                return Err(error);
            }       
        }
    }

    fn make_and_push_key_token(&mut self) -> Result<(), ParserError> {
        let mut msg = String::new();
        if self.get_current_container_type() != JsonContainerType::Dict {
            // current container must be dictionary
            msg = "Key is meaningful only in dictionary.".to_string();
        }
        // If previous token is key, emits error.
        if self.token_vector.len() > 0
                && self.token_vector.last().unwrap().get_type() == JsonTokenType::Key
        {
            msg = "Key cannot follow a key immediately.".to_string();
        }
        // Caution: JSON keys must be in delimited scheme
        if self.current_literal_scheme == LiteralScheme::Delimited {
            let key_token = tokenize_key(
                    &self.current_literal,
                    &self.parent_key[self.current_level as usize],
                    self.current_level,
                    self.current_line,
                    self.get_current_container_type(),
                    self.current_literal_scheme);
            self.push(key_token.clone());
            if let Err(error) = self.report_duplicate_key(key_token) {
                msg = error.full_message();
            }
        } else {
            msg = format!("Key '{}' isn't quoted. Keys in JSON must be enclosed
                    in double-quotation marks.", self.current_literal);
        }

        if msg != "" { // if error, throw.
            let mut error = ParserError::new(&msg, ErrorKind::SyntaxError);
            error.set_line(self.current_line);
            return Err(error);
        } else {
            // Set parent key for the next level.
            if self.parent_key.len() >= (self.current_level + 2) as usize {
                self.parent_key[(self.current_level + 1) as usize]
                        = self.current_literal.clone();
                return Ok(());
            } else {
                // This key becomes the parent key for the next level.
                self.parent_key.push(self.current_literal.clone());
                return Ok(());
            }
        }
    }

    fn per_iterable_container(&mut self) {
        if self.is_current_container_iterable() {
            if self.current_iter_index.len() == 0 {
                self.current_iter_index.push(0);
            }
            let name_string: String
                    = self.current_iter_index[self.current_level as usize]
                          .to_string().clone();
            let iter_key_token = tokenize_iter_key(
                &name_string,
                &self.parent_key[self.current_level as usize],
                self.current_level,
                self.current_line,
                self.get_current_container_type(),
                self.current_iter_index[self.current_level as usize]);
            self.push(iter_key_token);
            // update parent key by consuming `name_string`
            if self.parent_key.len() >= (self.current_level as usize) + 2 {
                self.parent_key[(self.current_level as usize) + 1] = name_string;
            } else {
                self.parent_key.push(name_string);
            }
        }
    }

    fn per_pushing_terminal_token(&mut self) -> Result<(), ParserError> {
        let veclen = self.token_vector.len();
        if !(self.token_vector[veclen - 1].get_type() == JsonTokenType::Key
                || self.token_vector[veclen - 1].get_type() == JsonTokenType::IterKey)
        {
            let mut error = ParserError::new(
                "A terminal value isn't preceeded by a key or an iteration key.", 
                ErrorKind::SyntaxError);
            error.set_line(self.current_line);
            Err(error)
        } else {
            // update previous key's referenced type
            self.token_vector[veclen - 1].referenced_type = "Terminal".to_string();
            Ok(())
        }
    }

    fn make_and_push_terminal_token(&mut self) -> Result<(), ParserError> {
        self.per_iterable_container();
        let terminal_type: JsonTerminalType;
        // available terminal types are dependent on parser type
        if self.parser_type == "json" {
            terminal_type = self.get_terminal_type(
                    &self.current_literal, self.current_literal_scheme)?;
        } else {
            terminal_type = self.get_terminal_plus_type(
                &self.current_literal,  self.current_literal_scheme)?;
        }
        let terminal_token = tokenize_terminal(
            &self.current_literal,
            &self.parent_key[(self.current_level as usize) + 1], // Note the level for PK
            self.current_level,
            self.current_line,
            self.get_current_container_type(),
            terminal_type,
            self.current_literal_scheme);
        self.per_pushing_terminal_token()?;
        self.push(terminal_token);
        Ok(())
    }
    
    /// TODO
    fn per_pushing_container_opening_token(&mut self, opening_symbol: char)
    -> Result<(), ParserError>
    {
        let mut msg = String::new();
        if self.token_vector.len() > 0 {
            let veclen = self.token_vector.len();
            // Case: curent container is Dict, but last token is NOT key.
            if self.get_current_container_type() == JsonContainerType::Dict
                && self.token_vector[veclen - 1].get_type() != JsonTokenType::Key
            {
                // curent container is Dict, but last token is NOT key.
                msg = "In dictionary, an object must be preceeded by a key.".to_string();
            } else if self.is_current_container_iterable()
                && self.token_vector[veclen - 1].get_type() != JsonTokenType::IterKey
            {
            // Case: current container is iterable, but last token is NOT iter-key.
                msg = "In a iterable container, any object must be preceeded \
                        by an iter key.".to_string();
            } else {
                // JSON allows either Dict or List container.
                if opening_symbol == symbol::L_BRACE {
                    self.token_vector[veclen - 1]
                        .referenced_type = "Dict".to_string();
                } else if opening_symbol == symbol::L_BRACKET {
                    self.token_vector[veclen - 1]
                        .referenced_type = "List".to_string();
                } else {}
            }
        }
        // Return Ok or Err
        if msg != "" {
            let mut error = ParserError::new(&msg, ErrorKind::SyntaxError);
            error.set_line(self.current_line);
            Err(error)
        } else {
            Ok(())
        }
    }

    fn make_and_push_container_opening_token(&mut self, opening_sym: char)
    -> Result<(), ParserError>
    {
        self.per_iterable_container();
        self.per_pushing_container_opening_token(opening_sym)?;
        let opening_token = tokenize(
            &String::from(opening_sym),
            get_token_type_by_symbol(opening_sym).unwrap(),
            &self.parent_key[(self.current_level as usize) + 1], // Note the level
            self.current_level,
            self.current_line,
            self.get_current_container_type() );
        self.push(opening_token);
        Ok(())
    }

    fn make_and_push_container_closing_token(&mut self, closing_sym: char) {
        // Before pushing closing token, adjust the iter-key at this level.
        if self.get_current_container_type() == JsonContainerType::List{
            self.current_iter_index[self.current_level as usize] = 0;
        }
        let closing_token = tokenize(
            &String::from(closing_sym),
            get_token_type_by_symbol(closing_sym).unwrap(),
            &self.parent_key[self.current_level as usize],
            self.current_level - 1,
            self.current_line,
            // Caution. Must use the container of the outer (higher) level, since
            // level switching is performed after pushing the closing token.
            self.current_container_type[(self.current_level as usize) - 1]);
        self.push(closing_token);
    }

    // Workflow methods

    fn per_space(&mut self) -> Result<(),()> {
        match self.is_delimited_string_open {
            true => {
                self.current_literal.push_str(" ");
                Ok(())
            },
            _ => Ok(())
        }
    }
    
    fn per_back_slash(&mut self) {
        match self.is_delimited_string_open {
            true => {
                match self.is_escaping {
                    true => {
                        self.current_literal.push_str("\\");
                        self.is_escaping = false;
                    },
                    false => self.is_escaping = true,
                }
            },
            false => self.current_literal.push_str("\\"), // append
        }
    }

    fn per_double_quotation(&mut self) {
    // respect two-character escaping sequence
        if self.current_literal == "" {
        // If current string literal is empty, double quote
        // is treated as a string-opening operator.
            if !self.is_delimited_string_open {
                self.is_delimited_string_open = true;
                self.current_literal_scheme = LiteralScheme::Delimited;
            } else {
                // This is the case of the second quotation mark of an empty
                // string. Just close it.
                self.is_delimited_string_open = false;
                self.latest_syntactic_operator = symbol::DOUBLE_QUOTE.to_string();
            }
        } else {
        // current literal is non-empty
            if self.is_delimited_string_open {
            // a delimited string has opened...
                if self.is_escaping { // previous char is escaping
                    // then double quote is a character of the string
                    // add double quote to the current literal.
                    self.current_literal.push_str(&symbol::DOUBLE_QUOTE.to_string());
                    // swith off escaping flag.
                    self.is_escaping = false;
                } else {
                    // then double quote is a string-closing operator
                    self.is_delimited_string_open = false;
                    // set double quote as the latest syntactic operator.
                    self.latest_syntactic_operator = symbol::DOUBLE_QUOTE.to_string();
                }
            } else {
                if self.current_literal_scheme == LiteralScheme::Liberal {
                    self.current_literal.push_str(&symbol::DOUBLE_QUOTE.to_string()); // append
                } else {
                    // If not LIBERAL scheme, the previous quoted string has been
                    // closed; this quotation mark starts a new delimited string.
                    // This algorithm allows a delimited string to be broken into
                    // multiple lines.
                    self.is_delimited_string_open = true;
                }
            }
        }
    }

    fn per_colon(&mut self) -> Result<(), ParserError> {
        let mut msg = String::new();
        if self.is_delimited_string_open {
            self.current_literal.push_str(&symbol::COLON.to_string());
            Ok(())
        } else {
            // Pre-tokenization syntax checks
            if self.is_first_noncomment_character() {
                if let Err(error) = self.illegal_first_noncomment_character(
                        &symbol::COLON.to_string()) {
                    msg = error.full_message();
                }
            }
            if self.is_current_container_iterable() {
                msg = "Colon operator is meaningless.".to_string();
            }
            if self.current_literal.len() == 0 {
                msg = "Colon operator must appear after
                      a non-empty key token.".to_string();
            }
            if self.latest_syntactic_operator == symbol::COLON.to_string() {
                msg = "Colon operator cannot appear immediately
                      after a colon operator.".to_string();
            }

            if msg != "" {
                let mut error = ParserError::new(&msg, ErrorKind::SyntaxError);
                error.set_line(self.current_line);
                Err(error)
            } else {
                if self.current_literal.len() > 0 {
                    if self.current_container_type[0] == JsonContainerType::None {
                        self.set_root_container_type(JsonContainerType::Dict)?; 
                    }
                    self.make_and_push_key_token()?;
                    self.reset_current_literal();
                    self.latest_syntactic_operator = symbol::COLON.to_string();
                }
                Ok(())
            }
        }
    }

    fn per_comma(&mut self) -> Result<(), ParserError> {
        let mut msg = String::new();
        if self.is_delimited_string_open {
            self.current_literal.push_str(&symbol::COMMA.to_string()); // append
            return Ok(());
        } else {
            // Syntax checks
            if self.is_first_noncomment_character() {
                if let Err(error) = self.illegal_first_noncomment_character(
                        &symbol::COMMA.to_string()) {
                    msg = error.full_message();
                }
            }
            if self.current_literal.len() == 0 {
                if !(self.latest_syntactic_operator == symbol::DOUBLE_QUOTE.to_string()
                        || self.latest_syntactic_operator == symbol::R_BRACE.to_string()
                        || self.latest_syntactic_operator == symbol::R_BRACKET.to_string())
                {
                    msg = "Comma can only appear after a terminal, \
                            a string, or a container closing operator.".to_string();
                }
            }
            if msg != "" {
                let mut error = ParserError::new(&msg, ErrorKind::SyntaxError);
                error.set_line(self.current_line);
                return Err(error);
            } else {
                if self.current_literal.len() > 0
                        || (self.current_literal.len() == 0
                            && self.latest_syntactic_operator == "\"") {
                    if self.current_container_type[0] == JsonContainerType::None {
                        self.set_root_container_type(JsonContainerType::List)?;
                    }
                    self.make_and_push_terminal_token()?;
                    self.reset_current_literal();
                    self.current_iter_index[self.current_level as usize] += 1;
                }
                // Post-pushing check
                let last_token_type = self.token_vector.last().unwrap().get_type();
                if !(last_token_type == JsonTokenType::Terminal
                        || last_token_type == JsonTokenType::DictClosing
                        || last_token_type == JsonTokenType::ListClosing) {
                    let mut error = ParserError::new(
                            &"Comma is meaningless.".to_string(),
                            ErrorKind::SyntaxError);
                    error.set_line(self.current_line);
                    return Err(error);
                } else {
                    self.latest_syntactic_operator = symbol::COMMA.to_string();
                    return Ok(());
                }
            }
        }
    }

    fn per_container_opening(&mut self, opening_sym: char)
    -> Result<(), ParserError>
    {
        let container_type = get_container_type_by_symbol(opening_sym).unwrap();
        if self.is_delimited_string_open {
            self.current_literal.push_str(&opening_sym.to_string()); // append
            return Ok(());
        } else {
            if self.is_first_noncomment_character() {
                if self.current_container_type[0] == JsonContainerType::None {
                    self.set_root_container_type(JsonContainerType::List)?;
                }
                if self.parent_key.len() == 1 {
                    let pk = self.parent_key[self.current_level as usize].clone();
                    self.parent_key.push(pk);
                }
            }
            // TODO Implment bracket match
            self.bracket_match.match_symbol(
                    opening_sym, self.current_line, self.current_column);
            // Must update container type for the next level.
            if self.current_container_type.len() >= (self.current_level as usize) + 2 {
                self.current_container_type[self.current_level as usize + 1] = container_type;
            } else {
                self.current_container_type.push(container_type);
            }
            self.make_and_push_container_opening_token(opening_sym)?;
            self.enter_deeper_level();
            self.current_iter_index.push(0);
            // update deepest level
            if self.current_level > self.deepest_level {
                self.deepest_level = self.current_level;
            }
            self.latest_syntactic_operator = opening_sym.to_string();
            return Ok(());
        }
    }

    fn per_container_closing(&mut self, closing_sym: char)
    -> Result<(), ParserError>
    {
        if self.is_delimited_string_open { // if delimited, append it to literal
            self.current_literal.push_str(&closing_sym.to_string());
            return Ok(());
        } else { // if liberal, it is closing token
            if self.is_first_noncomment_character() {
                self.illegal_first_noncomment_character(&closing_sym.to_string())?;
            }
            self.bracket_match.match_symbol(
                    closing_sym, self.current_line, self.current_column);
            // Current literal is not empty
            if self.current_literal.len() > 0 {
                self.make_and_push_terminal_token()?;
                self.reset_current_literal();
            }
            self.make_and_push_container_closing_token(closing_sym);
            // TODO
            if util::is_iterable_container(
                    &self.current_container_type[(self.current_level as usize) - 1].type_name()) {
                self.current_iter_index[self.current_level as usize] = 0;
                self.current_iter_index[(self.current_level as usize) - 1] += 1;
            }
            self.enter_higher_level();
            self.latest_syntactic_operator = closing_sym.to_string();
            return Ok(());
        }
    }
    
    fn per_other_symbol(&mut self, current_char: char) -> Result<(), ParserError> {
        // Current literal is empty, new literal collection starts...
        if self.current_literal.len() == 0 {
            // If inside delimited string is on, this is just after double
            // quotation mark, simply append the character
            if self.is_delimited_string_open {
                self.current_literal.push_str(&current_char.to_string());
            } else {
                self.current_literal_scheme = LiteralScheme::Liberal;
                self.current_literal.push_str(&current_char.to_string());
            }
            return Ok(());
        } else { // Current literal is non-empty
            if self.is_delimited_string_open == false
                && self.current_literal_scheme == LiteralScheme::Delimited
            {
                // A delimited string is before this character, for example
                // "first"name, i.e. appending a liberal string to a delimited
                // one is not allowed.
                let mut error = ParserError::new(
                    "Appending a liberal string to a delimited one is forbidden.",
                    ErrorKind::SyntaxError);
                error.set_line(self.current_line);
                return Err(error);
            } else {
                self.current_literal.push_str(&current_char.to_string());
                return Ok(());
            }
        }
    }

    // TODO
    fn per_end_of_file(&mut self) -> Result<(), ParserError> {
        if self.current_literal != "" {
            self.make_and_push_terminal_token()?;
        }
        self.reset_current_literal();
        return Ok(());
    }

    fn parse_line(&mut self, line: &str) -> Result<(), ParserError> {
        // Rust way of loop through a string
        for (index, current_char) in line.chars().enumerate() {
            self.current_column = index as i32;
            if current_char == symbol::BACK_SLASH {
                self.per_back_slash();
                continue;
            } else if current_char == symbol::DOUBLE_QUOTE {
                self.per_double_quotation();
                continue;
            } else if current_char == symbol::SPACE {
                self.per_space().unwrap();
                continue;
            } else if current_char == symbol::SHARP { // at # symbol
                if self.is_delimited_string_open {
                    self.current_literal.push_str(&current_char.to_string());
                } else {
                    break;
                }
                continue;
            } else if current_char == symbol::COLON {
                self.per_colon()?;
                continue;
            } else if current_char == symbol::COMMA {
                self.per_comma()?;
                continue;
            } else if util::is_opening_symbol(current_char) {
                self.per_container_opening(current_char)?;
                continue;
            } else if util::is_closing_symbol(current_char) {
                self.per_container_closing(current_char)?;
                continue;
            } else { // none of the above
                self.per_other_symbol(current_char)?;
                continue;
            }
        }
        Ok(())
    }
}

/// Public methods
impl JsonParser {
    // Parse a string that consists of multiple lines.
    pub fn parse_string(&mut self, src_string: &str) -> Result<(), ParserError> {
        for l in src_string.lines() {
            self.current_line += 1;
            self.current_column = 0;
            let line_copy = String::from(l);
            self.parse_line(&line_copy)?;
        }
        self.per_end_of_file()?;
        if !self.bracket_match.are_all_matched() {
            return Err(ParserError::new(
                "Unpaired brackets found.",
                ErrorKind::UnpairedBrackets));
        } else {
            return Ok(());
        }
    }
    
    // TODO
    pub fn parse_file(&mut self, filename: &str) -> Result<(), ParserError> {
        let path = Path::new(filename);
        // Open the path in read-only mode, returns `io::Result<File>`
        match File::open(&path) {
            Err(why) => {
                let error = ParserError::new(
                    &format!("Failed to open {}: {}", path.display(), why),
                    ErrorKind::FileNotFound);
                Err(error)
            },
            Ok(file) => {
                for res in io::BufReader::new(file).lines() {
                    self.current_line += 1;
                    self.current_column = 0;
                    match res {
                        Ok(line) => {
                            let line_copy = String::from(line);
                            self.parse_line(&line_copy)?;
                        },
                        Err(_) => continue
                    }
                }
                self.per_end_of_file()?;
                if !self.bracket_match.are_all_matched() {
                    let error = ParserError::new("Unpaired brackets found.",
                            ErrorKind::UnpairedBrackets);
                    Err(error)
                } else {
                    Ok(())
                }
            }
        }
    }

}

#[cfg(test)]
#[path = "./unittest/json_parser/tests.rs"]
mod tests;