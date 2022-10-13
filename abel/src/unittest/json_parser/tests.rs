//! Unittetst crate - json_parser
//! Crate location: /src/json_parser.rs
use super::*;

mod test_json_parser {
    use super::*;

    #[test]
    fn test_constructor() {
        let mut parser = JsonParser::new();
        assert_eq!(parser.current_column, 0);
        //parser.current_container_type.push("List".to_owned());
        parser.current_container_type.push(JsonContainerType::List);

        let literal = "MOV";
        let pk = "ROOT_KEY_";
        let level = 0;
        let line = 100;
        let container_type = JsonContainerType::Dict; //String::from("Dict");
        let scheme = LiteralScheme::Delimited; //"DELIMITED";

        let token = tokenize_key(literal, pk, level, line, container_type, scheme);
        if let Some(inner_vector) = parser.keys_per_level.last_mut() {
            inner_vector.push(token);
        }
        assert_eq!(parser.keys_per_level.len(), 1);
        assert_eq!(parser.keys_per_level[0].len(), 1);
        let token = &parser.keys_per_level[0][0];
        //assert_eq!(token.container_type, "Dict");
        assert_eq!(token.container_type, JsonContainerType::Dict);
    }

    #[test]
    /// Note that JSON has only null, double, boolean, and string types.
    fn test_data_conversion_method() {
        // NOTE: After reading from the external file, everything is stored as
        // string, only `scheme` argument tells how it is represented in the
        // file.
        let parser = JsonParser::new();
        let scheme = LiteralScheme::Liberal; //"LIBERAL";
        // Null type
        let test_string = "null";
        if let Ok(terminal_type) = parser.get_terminal_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Null");
        }
        // Bool type
        let test_string = "true";
        if let Ok(terminal_type) = parser.get_terminal_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Bool");
        }
        // Bool type
        let test_string = "false";
        if let Ok(terminal_type) = parser.get_terminal_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Bool");
        }
        // Double type
        let test_string = "0.10";
        if let Ok(terminal_type) = parser.get_terminal_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Double");
        }
        // Double type
        let test_string = "82871929.09091";
        if let Ok(terminal_type) = parser.get_terminal_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Double");
        }
        // Double type
        let test_string = "012832878";
        if let Ok(terminal_type) = parser.get_terminal_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Double");
        }

        // Default String type
        let test_string = "a0 .1 2832878";
        assert_eq!(
            parser.get_terminal_type(test_string, scheme).unwrap_err().get_header(),
            "SYNTAX_ERROR"
        );
    }


    #[test]
    fn test_parser_parse_line() {
        let mut parser = JsonParser::new();
        println!("{:?}", parser.current_iter_index);
        let test_single_line_string =
        "{ \
            \"MOV\": \"REGISTER\", \
            \"ADD\": 5, \
            \"SUB\": \"hello\" \
        }";
        //println!("{}", test_single_line_string);
        parser.parse_line(test_single_line_string).unwrap();
        for item in parser.get_token_vector() {
            println!("{} \n", item);
        }
    }
    /*
    #[test]
    fn test_parser_parse_file() {
        let mut parser = JSONParser::new();
        parser.parse_file("test_file.txt").unwrap();
        for item in parser.get_token_vector() {
            println!("{} \n", item);
        }
    }*/

}