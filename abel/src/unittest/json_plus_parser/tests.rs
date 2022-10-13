//! Unittetst crate - json_plus_parser
//! Crate location: /src/json_plus_parser.rs
//! 
//! NOTE! JSON+ parser is constructed via `JsonParser` with
//! a modified member.
use super::*;

mod test_json_plus_parser {
    use super::*;
    use crate::json_token::LiteralScheme;
    use crate::typefy::NamedType;

    #[test]
    fn test_make_function() {
        let test_parser = make_json_plus_parser();
        assert_eq!(test_parser.parser_type, "json_plus");
    }

    #[test]
    /// Note that JSON has only null, double, boolean, and string types.
    fn test_data_conversion_method() {
        // NOTE: After reading from the external file, everything is stored as
        // string, only `scheme` argument tells how it is represented in the
        // file.
        let parser = make_json_plus_parser();
        let scheme = LiteralScheme::Liberal; //"LIBERAL";
        // Null type
        let test_string = "null";
        if let Ok(terminal_type) = parser.get_terminal_plus_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Null");
        }
        // Bool type
        let test_string = "true";
        if let Ok(terminal_type) = parser.get_terminal_plus_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Bool");
        }
        // Bool type
        let test_string = "false";
        if let Ok(terminal_type) = parser.get_terminal_plus_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Bool");
        }
        // Double type
        let test_string = "0.10";
        if let Ok(terminal_type) = parser.get_terminal_plus_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Double");
        }
        // Double type
        let test_string = "82871929.09091";
        if let Ok(terminal_type) = parser.get_terminal_plus_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Double");
        }
        // Integer type
        let test_string = "012832878";
        if let Ok(terminal_type) = parser.get_terminal_plus_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Integer");
        }

        // Binary type
        let test_string = "0b0101010";
        if let Ok(terminal_type) = parser.get_terminal_plus_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Binary");
        }

        // Bitstring type
        let test_string = "_b010.1010";
        if let Ok(terminal_type) = parser.get_terminal_plus_type(test_string, scheme) {
            assert_eq!(terminal_type.type_name(), "Bitstring");
        }

        // Text type
        let test_string = "alpha_b010.1010";
        if let Ok(terminal_type) = parser.get_terminal_plus_type(test_string, LiteralScheme::Delimited) {
            assert_eq!(terminal_type.type_name(), "Text");
        }

        // Default String type
        let test_string = "a0 .1 2832878";
        assert_eq!(
            parser.get_terminal_type(test_string, scheme).unwrap_err().get_header(),
            "SYNTAX_ERROR"
        );
    }

    #[test]
    fn test_parser_parse_string() {
        let mut parser = JsonParser::new_plus();
        assert_eq!(parser.parser_type, "json_plus");

        let test_single_line_string =
        "{ \
            \"TXT1\": \"REGISTER\", \
            \"TXT2\": \"hello\", \
            \"INT\": 5, \
            \"DBL1\": 1e-5, \
            \"DBL2\": 0.099, \
            \"BIN\": 0b0101011, \
            \"BIS\": _b0101.011, \
        }";

        parser.parse_string(test_single_line_string).unwrap();
        for item in parser.get_token_vector() {
            println!("{} \n", item);
        }
    }
}