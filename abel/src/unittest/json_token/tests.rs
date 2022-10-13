//! Unittest crate json_token
//! Crate location /src/json_token.rs

use super::*;

mod test_json_token {
    use super::*;

    #[test]
    fn test_iter_key() {
        let literal = "MOV";
        let pk = "ROOT_KEY_";
        let level = 0;
        let line = 100;
        let container_type = JsonContainerType::List;
        let iter_index = 5;

        let token = tokenize_iter_key(literal, pk, level, line,
                                      container_type, iter_index);
        // customised fields
        assert_eq!(token.literal, literal);
        assert_eq!(token.literal, "MOV");

        assert_eq!(token.token_type.type_name(), "IterKey");

        assert_eq!(token.parent_key, pk);
        assert_eq!(token.parent_key, "ROOT_KEY_");

        assert_eq!(token.level, level);
        assert_eq!(token.level, 0);

        assert_eq!(token.line, line);
        assert_eq!(token.line, 100);

        assert_eq!(token.container_type, container_type);
        assert_eq!(token.container_type, JsonContainerType::List);

        assert_eq!(token.iter_index, iter_index);
        assert_eq!(token.iter_index, 5);

        // default fields
        assert_eq!(token.terminal_type, JsonTerminalType::NotSet);
        assert_eq!(token.literal_scheme, LiteralScheme::None);
        assert_eq!(token.referenced_type, "");
    }

    #[test]
    fn test_terminal() {
        let literal = "MOV";
        let pk = "ROOT_KEY_";
        let level = 0;
        let line = 100;
        let container_type = JsonContainerType::Dict; //String::from("Dict");
        let terminal_type = JsonTerminalType::Double; //"Integer";
        let scheme = LiteralScheme::Liberal; //"LIBERAL";

        let token = tokenize_terminal(literal, pk, level, line, container_type,
                                      terminal_type, scheme);
        // customised fields
        assert_eq!(token.literal, literal);
        assert_eq!(token.literal, "MOV");

        assert_eq!(token.token_type.type_name(), "Terminal");

        assert_eq!(token.parent_key, pk);
        assert_eq!(token.parent_key, "ROOT_KEY_");

        assert_eq!(token.level, level);
        assert_eq!(token.level, 0);

        assert_eq!(token.line, line);
        assert_eq!(token.line, 100);

        assert_eq!(token.container_type, container_type);
        assert_eq!(token.container_type, JsonContainerType::Dict);

        assert_eq!(token.terminal_type, JsonTerminalType::Double);
        assert_eq!(token.literal_scheme, scheme);

        // default fields
        assert_eq!(token.iter_index, -99);
        assert_eq!(token.referenced_type, "");
    }

    #[test]
    fn test_key() {
        let literal = "MOV";
        let pk = "ROOT_KEY_";
        let level = 0;
        let line = 100;
        let container_type = JsonContainerType::Dict; //String::from("Dict");
        let scheme = LiteralScheme::Delimited; //"DELIMITED";

        let token = tokenize_key(literal, pk, level, line,
                                 container_type, scheme);
        println!("{}", token);

        // customised fields
        assert_eq!(token.literal, literal);
        assert_eq!(token.literal, "MOV");

        assert_eq!(token.token_type.type_name(), "Key");

        assert_eq!(token.parent_key, pk);
        assert_eq!(token.parent_key, "ROOT_KEY_");

        assert_eq!(token.level, level);
        assert_eq!(token.level, 0);

        assert_eq!(token.line, line);
        assert_eq!(token.line, 100);

        assert_eq!(token.container_type, container_type);
        assert_eq!(token.container_type, JsonContainerType::Dict);
        assert_eq!(token.literal_scheme, scheme);

        // default fields
        assert_eq!(token.iter_index, -99);
        assert_eq!(token.referenced_type, "");
        assert_eq!(token.terminal_type, JsonTerminalType::NotSet);
    }
}
