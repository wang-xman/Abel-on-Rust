use crate::text::Text;
use crate::container::Container;
use crate::list::List;
use crate::dict::Dict;

use crate::util::get_closing_symbol_by_opening;
use crate::converter;
use crate::json_token::{JsonTokenType, JsonToken, JsonContainerType, JsonTerminalType};
use crate::json_parser::JsonParser;

pub struct JsonLoader {
    root_container_type: JsonContainerType,
    current_index: usize,
    global_dict: Dict,
    parser_type: String
}

/// Private methods
impl JsonLoader {
    fn is_matched_closing(&self, closing_token: &JsonToken,
                          opening_token: &JsonToken) -> bool
    {
        let opening_sym = opening_token.literal.chars().collect::<Vec<char>>()[0];
        let closing_sym = closing_token.literal.chars().collect::<Vec<char>>()[0];
        // That get_closing function returns Result<char, Error>
        closing_sym == get_closing_symbol_by_opening(opening_sym).unwrap()
            && closing_token.level == opening_token.level
            && closing_token.parent_key == opening_token.parent_key
    }

    fn make_root_dict(&mut self, token_vector: &Vec<JsonToken>) -> Box<Dict> {
        let mut dict_boxed = Box::new(Dict::new());
        while self.current_index < token_vector.len() {
            if token_vector[self.current_index].get_type() == JsonTokenType::Key {
                self.at_key(&mut dict_boxed, token_vector);
                continue;
            }
        }
        return dict_boxed;
    }

    fn make_root_list(&mut self, token_vector: &Vec<JsonToken>) -> Box<List> {
        let mut list_boxed = Box::new(List::new());
        while self.current_index < token_vector.len() {
            if token_vector[self.current_index].get_type() == JsonTokenType::IterKey {
                self.at_iter_key(&mut list_boxed, token_vector);
                continue;
            }
        }
        return list_boxed;
    }

    fn make_list(&mut self, index_opening_token: usize,
                 token_vector: &Vec<JsonToken>)
    -> Box<List>
    {
        let mut list_boxed = Box::new(List::new());
        self.current_index = index_opening_token + 1;
        while self.current_index < token_vector.len()
                && !self.is_matched_closing(
                    &token_vector[self.current_index],
                    &token_vector[index_opening_token])
        {
            if token_vector[self.current_index].get_type() == JsonTokenType::IterKey {
                self.at_iter_key(&mut list_boxed, token_vector);
                continue;
            }
        }
        self.current_index += 1;
        return list_boxed;
    }

    fn make_dict(&mut self, index_opening_token: usize,
                 token_vector: &Vec<JsonToken>) -> Box<Dict>
    {
        let mut dict_boxed = Box::new(Dict::new());
        // Prepare current index for iteration. Iteration start from next index.
        self.current_index = index_opening_token + 1;
        while self.current_index < token_vector.len()
                && !self.is_matched_closing(
                    &token_vector[self.current_index],
                    &token_vector[index_opening_token])
        {
            if token_vector[self.current_index].get_type() == JsonTokenType::Key {
                self.at_key(&mut dict_boxed, token_vector);
                continue;
            }
        }
        // Since when current index is pointing at a dict closing symbole, while
        // loop terminates, must update current index for the next root-level run.
        self.current_index += 1;
        return dict_boxed;
    }

    /// For standard JSON
    fn put_into<ContainerT, KeyT>(&mut self, boxed_ref: &mut Box<ContainerT>,
                                  key: KeyT, token: &JsonToken)
    where
        ContainerT: Container<KeyT>,
    {
        let value = token.literal.clone();
        if token.terminal_type == JsonTerminalType::Null {
            boxed_ref.set(key, converter::as_null(&value).unwrap()).unwrap();
        } else if token.terminal_type == JsonTerminalType::Bool {
            boxed_ref.set(key, converter::as_bool(&value).unwrap()).unwrap();
        } else if token.terminal_type == JsonTerminalType::Double {
            boxed_ref.set(key, converter::as_double(&value).unwrap()).unwrap();
        } else { // set item as Text
            boxed_ref.set(key, Text::new(&value)).unwrap();
        }
    }

    /// For JSON+ format
    fn put_plus_into<ContainerT, KeyT>(&mut self, boxed_ref: &mut Box<ContainerT>,
                                       key: KeyT, token: &JsonToken)
    where
        ContainerT: Container<KeyT>,
    {
        let value = token.literal.clone();
        if token.terminal_type == JsonTerminalType::Null {
            boxed_ref.set(key, converter::as_null(&value).unwrap()).unwrap();
        } else if token.terminal_type == JsonTerminalType::Bool {
            boxed_ref.set(key, converter::as_bool(&value).unwrap()).unwrap();
        } else if token.terminal_type == JsonTerminalType::Integer {
            boxed_ref.set(key, converter::as_integer(&value).unwrap()).unwrap();
        } else if token.terminal_type == JsonTerminalType::Double {
            boxed_ref.set(key, converter::as_double(&value).unwrap()).unwrap();
        } else if token.terminal_type == JsonTerminalType::Complex {
            boxed_ref.set(key, converter::as_complex(&value).unwrap()).unwrap();
        } else if token.terminal_type == JsonTerminalType::Binary {
            boxed_ref.set(key, converter::as_binary(&value).unwrap()).unwrap();
        } else if token.terminal_type == JsonTerminalType::Bitstring {
            boxed_ref.set(key, converter::as_bitstring(&value).unwrap()).unwrap();
        } else { // set item as Text
            boxed_ref.set(key, Text::new(&value)).unwrap();
        }
    }

    // TOOD
    fn fill_list(&mut self, list_boxed_ref: &mut Box<List>, iter_key: usize,
                 token_vector: &Vec<JsonToken>)
    {
         // Case 1, next token is a terminal
         if token_vector[self.current_index + 1].get_type()
                 == JsonTokenType::Terminal
        {
            // set terminal value
            let token = &token_vector[self.current_index + 1];
            if self.parser_type == "json" {
                self.put_into(list_boxed_ref, iter_key, token);
            } else {
                self.put_plus_into(list_boxed_ref, iter_key, token);
            }
            // shift current index to point at next iter key token
            self.current_index += 2;
        // Case 2, next token is dict opening.
        } else if token_vector[self.current_index + 1].get_type()
                == JsonTokenType::DictOpening
        {
            // build a dict recursively
            let subdict_boxed = self.make_dict(self.current_index + 1, token_vector);
            // set item: insert a dict into list
            list_boxed_ref.insert_box(iter_key, subdict_boxed);
        // Case 3, next token is list opening.
        } else if token_vector[self.current_index + 1].get_type()
                == JsonTokenType::ListOpening
        {
            // build a list
            let sublist_boxed = self.make_list(self.current_index + 1, token_vector);
            // set item: insert a list into list
            list_boxed_ref.insert_box(iter_key, sublist_boxed);
        } else { /* TODO */ }
    }

    fn fill_dict(&mut self, dict_boxed_ref: &mut Box<Dict>, key: &str,
                 token_vector: & Vec<JsonToken>)
    {
        // Case 1, next token is a terminal
        if token_vector[self.current_index + 1].get_type()
                == JsonTokenType::Terminal
        {
            // set terminal value
            let token = &token_vector[self.current_index + 1];
            if self.parser_type == "json" {
                self.put_into(dict_boxed_ref, key, token);
            } else {
                self.put_plus_into(dict_boxed_ref, key, token);
            }
            // shift current index to point at next iter key token
            self.current_index += 2;
        // Case 2, next token is dict opening.
        } else if token_vector[self.current_index + 1].get_type()
                == JsonTokenType::DictOpening
        {
            // build a dict recursively
            let subdict_boxed = self.make_dict(self.current_index + 1, token_vector);
            // set item: insert a dict into list
            dict_boxed_ref.insert_box(key, subdict_boxed);
        // Case 3, next token is list opening.
        } else if token_vector[self.current_index + 1].get_type()
                == JsonTokenType::ListOpening
        {
            // build a list
            let sublist_boxed = self.make_list(self.current_index + 1, token_vector);
            // set item: insert a list into list
            dict_boxed_ref.insert_box(key, sublist_boxed);
        } else { /* TODO */ }
    }

    fn at_key(&mut self, dict_boxed_ref: &mut Box<Dict>,
              token_vector: &Vec<JsonToken>)
    {
        let key = &token_vector[self.current_index].literal.clone();
        self.fill_dict(dict_boxed_ref, &key, token_vector);
    }

    fn at_iter_key(&mut self, list_boxed_ref: &mut Box<List>,
                   token_vector: & Vec<JsonToken>)
    {
        let iter_key = token_vector[self.current_index].literal.parse::<usize>().unwrap();
        self.fill_list(list_boxed_ref, iter_key, token_vector);
    }
}

/// Public methods
impl JsonLoader {
    pub fn load_from_parser(&mut self, parser: &JsonParser) {
        if self.parser_type != parser.parser_type {
            panic!("Parser type doesn't match!");
        }
        let token_vector = parser.get_token_vector();
        self.root_container_type = parser.get_root_container_type();
        if self.root_container_type == JsonContainerType::Dict { // root container is a dict
            let root_object_dict = self.make_root_dict(&token_vector);
            self.global_dict.insert_box("ROOT_KEY_", root_object_dict);
        } else if self.root_container_type == JsonContainerType::List { // root container is a list
            let root_object_list = self.make_root_list(&parser.get_token_vector()) ;
            self.global_dict.insert_box("ROOT_KEY_", root_object_list);
        } else {
            panic!("Root object type is undetermined.");
        }
    }

    pub fn load_from_file(&mut self, filename: &str) {
        if self.parser_type == "json" {
            let mut parser = JsonParser::new();
            parser.parse_file(filename).unwrap();
            self.load_from_parser(&parser);    
        } else { // JOSN+ need plus parser
            let mut parser = JsonParser::new_plus();
            parser.parse_file(filename).unwrap();
            self.load_from_parser(&parser);    
        }
    }

    pub fn get_global_dict(&self) -> &Dict {
        &self.global_dict
    }
}

/// Public methods
impl JsonLoader {
    pub fn new() -> Self {
        Self {
            root_container_type: JsonContainerType::None,
            current_index: 0,
            global_dict: Dict::new(),
            parser_type: String::from("json")
        }
    }

    // Returns a loader for JSON plus format
    pub fn new_plus() -> Self {
        Self {
            root_container_type: JsonContainerType::None,
            current_index: 0,
            global_dict: Dict::new(),
            parser_type: String::from("json_plus")
        }
    }
}

#[cfg(test)]
#[path = "./unittest/json_loader/tests.rs"]
mod tests;