//! Unittest crate json_loader
//! 
//! Note that the current directory is one level above /src.
//! This information is important for file loading.
use super::*;
use crate::object::Object;
use crate::bool::Bool;
use crate::integer::Integer;
use crate::double::Double;
use crate::complex::Complex;
use crate::text::Text;
use crate::binary::Binary;
use crate::bitstring::Bitstring;
use crate::container::Container;

mod test_json_loader_from_string {
    use super::*;

    #[test]
    fn test_json_loader_from_local_string() {
        let test_single_line_string =
        "{ \
            \"STR\": \"REGISTER\", \
            \"DBL\": 5, \
            \"BOOL\": true \
        }";
        let mut parser = JsonParser::new();
        parser.parse_string(test_single_line_string).unwrap();

        let mut json_loader = JsonLoader::new();
        assert_eq!(json_loader.parser_type, "json");
        assert_eq!(json_loader.parser_type, parser.parser_type);
        json_loader.load_from_parser(&parser);

        /* Parsed root dictionary shall be the following
        {
            ROOT_KEY_: [
                {
                    MOV: "REGISTER",
                    ADD: 5,
                    SUB: "hello"
                }
            ]
        }
        */
        // Root dictionary created
        assert_eq!(json_loader.global_dict.type_name(), "Dict");
        // ROOT_KEY_ is pointing to a list
        assert!(
            json_loader.global_dict.get_ref::<List>("ROOT_KEY_").unwrap().as_any().is::<List>()
        );
        // Downcast to a List
        let root_list = json_loader.global_dict.get_ref::<List>("ROOT_KEY_")
                        .unwrap()
                        .as_any()
                        .downcast_ref::<List>()
                        .unwrap();
        // List has one element
        assert_eq!(root_list.len(), 1);
        // Acquire the element in the list
        let target_dict = root_list[0]
                          .as_any()
                          .downcast_ref::<Dict>()
                          .unwrap();
        // Analyse the dict
        assert!(target_dict.get_ref::<Text>("STR").unwrap().as_any().is::<Text>());
        assert!(target_dict.get_ref::<Double>("DBL").unwrap().as_any().is::<Double>());
        assert!(target_dict.get_ref::<Bool>("BOOL").unwrap().as_any().is::<Bool>());
    }

}


// To test private method of JSON loader, this module must be placed here.
mod test_json_plus_load_local_string {
    use super::*;

    #[test]
    fn test_json_plus_loader() {
        let test_single_line_string =
        "{ \
            \"STR\": \"REGISTER\", \
            \"INT\": 5, \
            \"BOOL\": true, \
            \"COMP\": 0.005+1.0e-10j, \
            \"BIN\": 0b101010, \
            \"BIS\": _b101.010, \
            \"DICT\": { \
                \"DBL\": 0.5, \
            } \
        }";
        let mut plus_parser = JsonParser::new_plus();
        plus_parser.parse_string(test_single_line_string).unwrap();

        let mut json_plus_loader = JsonLoader::new_plus();
        assert_eq!(json_plus_loader.parser_type, "json_plus");
        assert_eq!(json_plus_loader.parser_type, plus_parser.parser_type);
        json_plus_loader.load_from_parser(&plus_parser);

        // Downcast to a List
        let root_list = json_plus_loader.global_dict.get_ref::<List>("ROOT_KEY_")
                        .unwrap()
                        .as_any()
                        .downcast_ref::<List>()
                        .unwrap();
        // List has one element
        assert_eq!(root_list.len(), 1);
        // Acquire the element in the list
        let target_dict = root_list[0]
                          .as_any()
                          .downcast_ref::<Dict>()
                          .unwrap();
        // Analyse the dict
        assert!(target_dict.get_ref::<Text>("STR").unwrap().as_any().is::<Text>());
        assert!(target_dict.get_ref::<Integer>("INT").unwrap().as_any().is::<Integer>());
        assert!(target_dict.get_ref::<Complex>("COMP").unwrap().as_any().is::<Complex>());
        assert!(target_dict.get_ref::<Bool>("BOOL").unwrap().as_any().is::<Bool>());
        assert!(target_dict.get_ref::<Binary>("BIN").unwrap().as_any().is::<Binary>());
        assert!(target_dict.get_ref::<Bitstring>("BIS").unwrap().as_any().is::<Bitstring>());
        assert!(target_dict.get_ref::<Dict>("DICT").unwrap().as_any().is::<Dict>());
        // NOTE! Method get_ref returns reference!
        assert_eq!(target_dict.get_ref::<Text>("STR").unwrap(), "REGISTER");
        assert_eq!(*target_dict.get_ref::<Integer>("INT").unwrap(), 5);
        assert_eq!(*target_dict.get_ref::<Bool>("BOOL").unwrap(), true);
        assert_eq!(*target_dict.get_ref::<Binary>("BIN").unwrap(), Binary::from("0b101010"));
        // test complex number
        assert_eq!((*target_dict.get_ref::<Complex>("COMP").unwrap()).real(), 0.005);
        assert_eq!((*target_dict.get_ref::<Complex>("COMP").unwrap()).imag(), 1.0e-10);

        let dict_ref: &Dict = target_dict.get_ref::<Dict>("DICT").unwrap();
        assert_eq!(*dict_ref.get_ref::<Double>("DBL").unwrap(), 0.5);
    }
}


mod test_json_loader_from_file {
    use super::*;

    #[test]
    fn test_loader_from_file() {
        let mut json_loader = JsonLoader::new();
        // Current directory is one level above /src
        json_loader.load_from_file("./src/unittest/test_files/standard_json.json");
        // Root dictionary created
        assert_eq!(json_loader.get_global_dict().type_name(), "Dict");

        // Acquire global dictionary
        let global = json_loader.get_global_dict();
        // Inside global dictionary "ROOT_KEY_" is pointing to a list
        assert!(
            global.get_ref::<List>("ROOT_KEY_").unwrap().as_any().is::<List>()
        );
        // Downcast to a List
        let root_list = global.get_ref::<List>("ROOT_KEY_").unwrap();
        // List has one element
        assert_eq!(root_list.len(), 1);
        // Acquire the element in the list
        let target_dict = root_list.get_ref::<Dict>(0).unwrap();
        // Analyse the dict
        assert!(target_dict.get_ref::<Text>("STR").unwrap().as_any().is::<Text>());
        assert!(target_dict.get_ref::<Double>("DBL").unwrap().as_any().is::<Double>());
        assert!(target_dict.get_ref::<Bool>("BOOL").unwrap().as_any().is::<Bool>());
        assert!(target_dict.get_ref::<List>("LIST").unwrap().as_any().is::<List>());
    }

}

mod test_json_plus_loader_from_file {
    use super::*;

    #[test]
    fn test_loader_from_file() {
        let mut json_loader = JsonLoader::new_plus();
        // Current directory is one level above /src
        json_loader.load_from_file("./src/unittest/test_files/json_plus.abel");
        // Root dictionary created
        assert_eq!(json_loader.get_global_dict().type_name(), "Dict");

        // Acquire global dictionary
        let global = json_loader.get_global_dict();
        // Inside global dictionary "ROOT_KEY_" is pointing to a list
        assert!(
            global.get_ref::<List>("ROOT_KEY_").unwrap().as_any().is::<List>()
        );
        // Downcast to a List
        let root_list = global.get_ref::<List>("ROOT_KEY_").unwrap();
        // List has one element
        assert_eq!(root_list.len(), 1);

        // Acquire the element in the list, which is a dictionary
        let target_dict = root_list.get_ref::<Dict>(0).unwrap();
        // Analyse the dict
        assert!(target_dict.get_ref::<Text>("TEXT1").unwrap().as_any().is::<Text>());
        assert!(target_dict.get_ref::<Integer>("INT1").unwrap().as_any().is::<Integer>());
        assert!(target_dict.get_ref::<Double>("DBL1").unwrap().as_any().is::<Double>());
        assert!(target_dict.get_ref::<Double>("DBL2").unwrap().as_any().is::<Double>());
        assert!(target_dict.get_ref::<Complex>("CPL1").unwrap().as_any().is::<Complex>());
        assert!(target_dict.get_ref::<Complex>("CPL2").unwrap().as_any().is::<Complex>());
        assert!(target_dict.get_ref::<Bool>("BOOL").unwrap().as_any().is::<Bool>());
        assert!(target_dict.get_ref::<List>("LIST").unwrap().as_any().is::<List>());
    }

}