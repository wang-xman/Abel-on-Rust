use crate::json_parser::JsonParser;

pub fn make_json_plus_parser() -> JsonParser {
    JsonParser::new_plus()
}

#[cfg(test)]
#[path = "./unittest/json_plus_parser/tests.rs"]
mod tests;