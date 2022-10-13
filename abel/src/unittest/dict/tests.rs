//! Unittest - crate dict
//! Crate location: /src/dict.rs
use super::*;

mod test_dict_contructor {
    use super::*;
    use crate::integer::Integer;
    use crate::double::Double;
    //use crate::list::List;
    use crate::text::Text;

    #[test]
    fn test_empty_dict() {
        let dict = Dict::new();
        assert!(!dict.has_key("k1"));
        assert!(!dict.has_key("00"));
    }

    #[test]
    fn test_constructor() {
        let mut test_dict = Dict::new();
        test_dict.insert("k1", Integer::new(10));
        // Another key
        let k2 = "KEY2";
        test_dict.insert(k2, Double::new(10.0));
        assert!(test_dict.has_key("k1"));
        assert!(test_dict.has_key("KEY2"));
        test_dict.insert("key_3", Text::new("Just a test!"));
        assert!(test_dict.has_key("key_3"));
    }
}

mod test_dict_has_key_verifier {
    use super::*;
    use crate::integer::Integer;
    use crate::double::Double;
    use crate::list::List;
    use crate::text::Text;
    
    #[test]
    fn test_has_key() {
        let mut test_dict = Dict::new();
        test_dict.insert("k1", Integer::new(10));
        test_dict.insert("k2", Double::new(0.9));
        test_dict.insert("k3", List::new());
        test_dict.insert("k4", Text::new("Another test."));

        assert!(test_dict.has_key("k1"));
        assert!(test_dict.has_key("k2"));
        assert!(test_dict.has_key("k3"));
        assert!(test_dict.has_key("k4"));
        // following keys don't exist
        assert!(!test_dict.has_key("k34"));
        assert!(!test_dict.has_key("key"));
        assert!(!test_dict.has_key("0987hy"));
    }
}

mod test_dict_own_get {
    use super::*;
    use crate::integer::Integer;
    use crate::double::Double;
    //use crate::list::List;
    use crate::text::Text;

    #[test]
    fn test_get() {
        let mut dict = Dict::new();
        dict.insert("k1", Integer::new(10));
        dict.insert("k2", Double::new(0.9));
        // make a sub-dict and insert it into dict
        let mut subdict = Dict::new();
        subdict.insert("subk1", Text::new("Hello"));
        subdict.insert("subk2", Integer::new(50));
        dict.insert("k3", subdict);

        assert_eq!(dict.get("k1").unwrap().type_name(), "Integer");
        assert_eq!(dict.get("k2").unwrap().type_name(), "Double");
        assert_eq!(dict.get("k3").unwrap().type_name(), "Dict");
    }
}

mod test_dict_container_trait_methods {
    use super::*;
    use crate::integer::Integer;
    use crate::double::Double;
    use crate::list::List;
    use crate::text::Text;
    use crate::marker::ScalarValued;

    #[test]
    fn test_get_type_name() {
        let mut test_dict = Dict::new();
        test_dict.insert("k1", Integer::new(10));
        test_dict.insert("k2", Double::new(0.9));

        let mut subdict = Dict::new();
        subdict.insert("subk1", Text::new("Hello"));
        subdict.insert("subk2", Integer::new(50));
        test_dict.insert("k3", subdict);

        // keys at highest level
        assert!(test_dict.has_key("k1"));
        assert!(test_dict.has_key("k2"));
        assert!(test_dict.has_key("k3"));
        // type names at highest level
        assert!(test_dict.get_type_name("k1").unwrap() == "Integer");
        assert!(test_dict.get_type_name("k2").unwrap() == "Double");
        assert!(test_dict.get_type_name("k3").unwrap() == "Dict");
        // keys at inner level
        let subdict = test_dict.get_ref::<Dict>("k3").unwrap();
        assert!(subdict.has_key("subk1"));
        assert!(subdict.has_key("subk2"));
        // type names at inner level
        assert!(subdict.get_type_name("subk1").unwrap() == "Text");
        assert!(subdict.get_type_name("subk2").unwrap() == "Integer");
    }

    #[test]
    fn test_set() {
        let mut test_dict = Dict::new();
        match test_dict.set("k1", Integer::new(10)) {
            Ok(_) => {},
            Err(_) => panic!("You shouldn't see this message!")
        }

        match test_dict.set("k2", Double::new(0.9)) {
            Ok(_) => {},
            Err(_) => panic!("You shouldn't see this message!")
        }

        let mut subdict = Dict::new();
        match subdict.set("subk1", Text::new("Hello")) {
            Ok(_) => {},
            Err(_) => panic!("You shouldn't see this message!")
        }

        match subdict.set("subk2", Integer::new(50)) {
            Ok(_) => {},
            Err(_) => panic!("You shouldn't see this message!")
        }

        match test_dict.set("k3", subdict) {
            Ok(_) => {},
            Err(_) => panic!("You shouldn't see this message!")
        }
    }

    #[test]
    fn test_set_box() {
        let mut test_dict = Dict::new();
        match test_dict.set_box("k1", Box::new(Integer::new(10))) {
            Ok(_) => {},
            Err(_) => panic!("You shouldn't see this message!")
        }

        match test_dict.set_box("k2", Box::new(Double::new(0.9))) {
            Ok(_) => {},
            Err(_) => panic!("You shouldn't see this message!")
        }

        let mut subdict = Dict::new();
        match subdict.set_box("subk1", Box::new(Text::new("Hello"))) {
            Ok(_) => {},
            Err(_) => panic!("You shouldn't see this message!")
        }

        match subdict.set_box("subk2", Box::new(Integer::new(50))) {
            Ok(_) => {},
            Err(_) => panic!("You shouldn't see this message!")
        }

        match test_dict.set_box("k3", Box::new(subdict)) {
            Ok(_) => {},
            Err(_) => panic!("You shouldn't see this message!")
        }
    }
    // Directly set a primitive type in dictionary.
    #[test]
    fn test_set_from() {
        let mut dict = Dict::new();
        // direct
        dict.set_from("k1", 10).unwrap();
        dict.set_from("k2", 0.09).unwrap();
        dict.set_from("k3", "Hello").unwrap();
        // type names
        assert!(dict.get_type_name("k1").unwrap() == "Integer");
        assert!(dict.get_type_name("k2").unwrap() == "Double");
        assert!(dict.get_type_name("k3").unwrap() == "Text");
        // get ref and compare
        assert_eq!(*dict.get_ref::<Integer>("k1").unwrap(), 10);
        assert_eq!(*dict.get_ref::<Double>("k2").unwrap(), 0.09);
        assert_eq!(*dict.get_ref::<Text>("k3").unwrap(), "Hello");
    }

    #[test]
    fn test_set_chain() {
        let mut test_dict = Dict::new();
        test_dict.set("k1", Integer::new(10)).unwrap()
                 .set("k2", Double::new(0.9)).unwrap()
                 .set("k3", Text::new("Hello")).unwrap();
        
        // Error. Key "k3" exists already
        match test_dict.set("k3", Text::new("World")) {
            Ok(_) => panic!("You shouldn't see this!"),
            Err(err) => println!("{}", err.full_message())
        }
    }

    #[test]
    fn test_chained_set_from() {
        let mut dict = Dict::new();
        dict.set_from("k1", 10).unwrap()
            .set_from("k2", 0.9).unwrap()
            .set_from("k3", "Hello").unwrap();
        // get ref and compare
        assert_eq!(*dict.get_ref::<Integer>("k1").unwrap(), 10);
        assert_eq!(*dict.get_ref::<Double>("k2").unwrap(), 0.9);
        assert_eq!(*dict.get_ref::<Text>("k3").unwrap(), "Hello");
    }

    #[test]
    fn test_set_then_get() {
        let mut test_dict = Dict::new();

        let double = test_dict.set("k1", Integer::new(10)).unwrap()
                 .set("k2", Double::new(0.9)).unwrap()
                 .set("k3", Dict::new()).unwrap()
                 .get_ref::<Double>("k2").unwrap();
        assert_eq!(*double, 0.9);
    }

    #[test]
    fn test_key_value() {
        let mut test_dict = Dict::new();
        let int1 = Integer::new(10);
        // Caution! Object `int1` has been moved into the dictionary.
        test_dict.insert("k1", int1);

        // create a list
        let mut list = List::new();
        let test_string = String::from("Test string");
        let rstring = Text::new(&test_string);
        // push one item into the list
        list.push(rstring);
        test_dict.insert("k2", list);

        // Access an Integer
        match test_dict.get("k1") {
            Some(ref_to_box) => {
                assert!(ref_to_box.as_any().is::<Integer>());
                match ref_to_box.as_any().downcast_ref::<Integer>() {
                    Some(ref_to_type) => {
                        assert_eq!(ref_to_type.value(), 10);
                    },
                    _ => {},
                }
            },
            _ => println!("Not integer") 
        }

        // Access a List inside the dictionary
        match test_dict.get("k2") {
            Some(ref_to_box) => {
                assert!(ref_to_box.as_any().is::<List>());
                match ref_to_box.as_any().downcast_ref::<List>() {
                    Some(ref_to_type) => {
                        // list has one item
                        assert_eq!(ref_to_type.len(), 1);
                        // only item in list is a Rstring
                        assert!(ref_to_type[0].as_any().is::<Text>());
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    #[test]
    fn test_get_ref() {
        // Create an empty Dictionary
        let mut test_dict = Dict::new();

        // Insert an integer.
        // Caution! Object `int1` has been moved into the dictionary.
        let int1 = Integer::new(10);
        test_dict.insert("int0", int1);

        // Create a list and push an some text into it
        let mut list = List::new();
        let text = Text::new("Test string");
        // Move one item into the list
        list.push(text);
        assert_eq!(list.len(), 1);
        // move list into dictionary.
        test_dict.insert("list1", list);

        // Read out.
        // Integer
        let int0_ref = test_dict.get_ref::<Integer>("int0").unwrap();
        assert_eq!(int0_ref.value(), 10);

        // Get the list
        let list_ref = test_dict.get_ref::<List>("list1").unwrap();
        // list has only one item
        assert_eq!(list_ref.len(), 1);
        // that item is a Text object.
        let text_ref = list_ref.get_ref::<Text>(0).unwrap();
        assert_eq!(text_ref.value(), "Test string");
    }

    #[test]
    fn test_get_mut_ref() {
        // Create an empty Dictionary
        let mut test_dict = Dict::new();

        // 1. Insert an integer.
        // Caution! Object `int1` has been moved into the dictionary.
        let int1 = Integer::new(10);
        test_dict.insert("int0", int1);

        // 2. Create a list and push an some text into it
        let mut list = List::new();
        let text = Text::new("Test string");
        // Move one item into the list
        list.push(text);
        assert_eq!(list.len(), 1);
        // move list into dictionary.
        test_dict.insert("list1", list);

        // Read out and insert into list
        let sublist = List::new();
        let mut subdict = Dict::new();
        subdict.insert("text", Text::new("Hola"));
        // push these two items into list
        let list_ref = test_dict.get_mut_ref::<List>("list1").unwrap();
        list_ref.push(sublist);
        list_ref.push(subdict);
        // now this list has 3 items.
        assert_eq!(list_ref.len(), 3);

        // Let's read out key-value in the dict sitting in the list
        let dict = list_ref.get_mut_ref::<Dict>(2).unwrap();
        assert_eq!(dict.get_ref::<Text>("text").unwrap().value(), "Hola");
        // since it is a mutable borrow, let's insert something.
        dict.insert("int2", Integer::new(0));
    }
}