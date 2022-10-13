//! Unittest json_plus_loader
//! Note that private method of json_loader are tested here.
//! See unittest for json_loader for the tests of private methods.
use super::*;
use crate::object::Object;
use crate::bool::Bool;
use crate::integer::Integer;
use crate::double::Double;
use crate::complex::Complex;
use crate::text::Text;
use crate::container::Container;
use crate::list::List;
use crate::dict::Dict;

mod test_json_plus_loader {
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