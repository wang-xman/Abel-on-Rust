//! Unittest crate list
//! Crate location: /src/list.rs
use super::*;

// List constructor can construct a list instance from a slice.
mod test_list_constructor_from_slice {
    use super::*;
    use crate::{bool::Bool, integer::Integer, double::Double, text::Text};
    use crate::typefy::Downcast;

    #[test]
    fn test_construct_from_slice_bool() {
        let slice = [true, true, false, true, false];
        let list = List::from_slice(&slice);

        assert_eq!(list.len(), 5);
        for (i, item) in list.iter().enumerate() {
            match i {
                0 => {
                    assert_eq!("Bool", item.type_name());
                    let var = item.to_ref::<Bool>().unwrap();
                    assert_eq!(*var, true);
                }
                1 => {
                    assert_eq!("Bool", item.type_name());
                    let var_ref = item.to_ref::<Bool>().unwrap();
                    assert_eq!(*var_ref, true);
                },
                2 => {
                    assert_eq!("Bool", item.type_name());
                    let var_ref = item.to_ref::<Bool>().unwrap();
                    assert_eq!(*var_ref, false);
                },
                3 => assert_eq!("Bool", item.type_name()),
                _ => {}
            }
        }
    }

    #[test]
    fn test_construct_from_slice_i32() {
        let slice = [0,1,2,3,4];
        let list = List::from_slice(&slice);

        assert_eq!(list.len(), 5);
        for (i, item) in list.iter().enumerate() {
            match i {
                0 => {
                    assert_eq!("Integer", item.type_name());
                    let var = item.to_ref::<Integer>().unwrap(); // will panic if not using Integer
                    assert_eq!(*var, 0);
                }
                1 => {
                    assert_eq!("Integer", item.type_name());
                    let var_ref = item.to_ref::<Integer>().unwrap(); // will panic if not using Integer
                    assert_eq!(*var_ref, 1);
                },
                2 => {
                    assert_eq!("Integer", item.type_name());
                    let var_ref = item.to_ref::<Integer>().unwrap(); // will panic if not using Integer
                    assert_eq!(*var_ref, 2);
                },
                3 => assert_eq!("Integer", item.type_name()),
                _ => {}
            }
        }
    }

    #[test]
    fn test_construct_from_slice_f64() {
        let slice = [0.0, 0.1, 0.2, 0.3, 0.4];
        let list = List::from_slice(&slice);

        assert_eq!(list.len(), 5);
        for (i, item) in list.iter().enumerate() {
            match i {
                0 => {
                    assert_eq!("Double", item.type_name());
                    let var = item.to_ref::<Double>().unwrap(); // will panic if not using Integer
                    assert_eq!(*var, 0.0);
                }
                1 => {
                    assert_eq!("Double", item.type_name());
                    let var_ref = item.to_ref::<Double>().unwrap(); // will panic if not using Integer
                    assert_eq!(*var_ref, 0.1);
                },
                2 => {
                    assert_eq!("Double", item.type_name());
                    let var_ref = item.to_ref::<Double>().unwrap(); // will panic if not using Integer
                    assert_eq!(*var_ref, 0.2);
                },
                3 => assert_eq!("Double", item.type_name()),
                _ => {}
            }
        }
    }

    #[test]
    fn test_construct_from_slice_str() {
        let slice = ["0.0", "0.1", "0.2", "0.3", "0.4"];
        let list = List::from_slice(&slice);

        assert_eq!(list.len(), 5);
        for (i, item) in list.iter().enumerate() {
            match i {
                0 => {
                    assert_eq!("Text", item.type_name());
                    let var = item.to_ref::<Text>().unwrap();
                    assert_eq!(*var, "0.0");
                }
                1 => {
                    assert_eq!("Text", item.type_name());
                    let var_ref = item.to_ref::<Text>().unwrap();
                    assert_eq!(*var_ref, "0.1");
                },
                2 => {
                    assert_eq!("Text", item.type_name());
                    let var_ref = item.to_ref::<Text>().unwrap();
                    assert_eq!(*var_ref, "0.2");
                },
                3 => assert_eq!("Text", item.type_name()),
                _ => {}
            }
        }
    }

    #[test]
    fn test_construct_from_slice_string() {
        let slice = [String::from("0.0"), String::from("0.1"), String::from("0.2")];
        let list = List::from_slice(&slice);

        assert_eq!(list.len(), 3);
        for (i, item) in list.iter().enumerate() {
            match i {
                0 => {
                    assert_eq!("Text", item.type_name());
                    let var = item.to_ref::<Text>().unwrap();
                    assert_eq!(*var, "0.0");
                }
                1 => {
                    assert_eq!("Text", item.type_name());
                    let var_ref = item.to_ref::<Text>().unwrap();
                    assert_eq!(*var_ref, "0.1");
                },
                2 => {
                    assert_eq!("Text", item.type_name());
                    let var_ref = item.to_ref::<Text>().unwrap();
                    assert_eq!(*var_ref, "0.2");
                },
                3 => assert_eq!("Text", item.type_name()),
                _ => {}
            }
        }
    }
}

// Test method `push`
mod test_list_push {
    use super::*;
    use crate::{bool::Bool, integer::Integer, double::Double, text::Text};
    use crate::typefy::Downcast;
    
    // Push primitive types
    #[test]
    fn test_mixed_primitive_scalar_types() {
        let mut list = List::new();
        list.push_from(0);
        list.push_from(1.0);
        list.push_from(true);
        list.push_from(3);
        list.push_from(4);

        assert_eq!(list.len(), 5);
        for (i, item) in list.iter().enumerate() {
            match i {
                0 => {
                    assert_eq!("Integer", item.type_name());
                    let var = item.to_ref::<Integer>().unwrap();
                    assert_eq!(*var, 0);
                }
                1 => {
                    assert_eq!("Double", item.type_name());
                    let var_ref = item.to_ref::<Double>().unwrap();
                    assert_eq!(*var_ref, 1.0);
                },
                2 => {
                    assert_eq!("Bool", item.type_name());
                    let var_ref = item.to_ref::<Bool>().unwrap();
                    assert_eq!(*var_ref, true);
                },
                3 => assert_eq!("Integer", item.type_name()),
                _ => {}
            }
        }

    }

    // Push slice of primitive types
    #[test]
    fn test_push_slice_of_i32() {
        let mut list = List::new();
        list.push_from_slice(&[0, 1, 2, 3, 4]);
        assert_eq!(list.len(), 5);
        for (i, item) in list.iter().enumerate() {
            match i {
                0 => {
                    assert_eq!("Integer", item.type_name());
                    let var_ref = item.to_ref::<Integer>().unwrap();
                    assert_eq!(*var_ref, 0);
                },
                1 => assert_eq!("Integer", item.type_name()),
                2 => assert_eq!("Integer", item.type_name()),
                3 => assert_eq!("Integer", item.type_name()),
                //4 => assert_eq!("Integer", item.type_name()),
                _ => {}
            }
        }
    }

    #[test]
    fn test_push_slice_of_f64() {
        let mut list = List::new();
        list.push_from_slice(&[0.0, 1.0, 2.0, 3.0, 4.0]);
        assert_eq!(list.len(), 5);
        for (i, item) in list.iter().enumerate() {
            match i {
                0 => {
                    assert_eq!("Double", item.type_name());
                    let var_ref = item.to_ref::<Double>().unwrap();
                    assert_eq!(*var_ref, 0.0);
                },
                1 => assert_eq!("Double", item.type_name()),
                2 => assert_eq!("Double", item.type_name()),
                3 => assert_eq!("Double", item.type_name()),
                //4 => assert_eq!("Integer", item.type_name()),
                _ => {}
            }
        }
    }

    #[test]
    fn test_push_slice_of_str() {
        let mut list = List::new();
        list.push_from_slice(&["Alpha", "Beta", "Delta", "Gamma"]);
        assert_eq!(list.len(), 4);
        for (i, item) in list.iter().enumerate() {
            match i {
                0 => {
                    assert_eq!("Text", item.type_name());
                    let var_ref = item.to_ref::<Text>().unwrap();
                    assert_eq!(*var_ref, "Alpha");
                },
                1 => assert_eq!("Text", item.type_name()),
                2 => assert_eq!("Text", item.type_name()),
                3 => assert_eq!("Text", item.type_name()),
                //4 => assert_eq!("Integer", item.type_name()),
                _ => {}
            }
        }
    }
}

// List in a for loop and iterator.
mod test_list_in_for_loop {
    use super::*;
    use crate::{bool::Bool, null::Null, text::Text};

    #[test]
    fn test_enumerate() {
        let mut test_list = List::new();
        test_list.push(Bool::new(false));
        test_list.push(Null::new());
        test_list.push(Text::new(&String::from("Test string")));
        test_list.push(List::new());

        for (i, item) in test_list.iter().enumerate() {
            match i {
                0 => assert_eq!("Bool", item.type_name()),
                1 => assert_eq!("Null", item.type_name()),
                2 => assert_eq!("Text", item.type_name()),
                3 => assert_eq!("List", item.type_name()),
                //4 => assert_eq!("Integer", item.type_name()),
                _ => {}
            }
        }
        assert_eq!(test_list.len(), 4);
    }
}

// Push Rabel internal types into a list
mod test_list_push_internal_types {
    use super::*;
    use crate::{bool::Bool, null::Null, text::Text};
    //use crate::marker::ScalarValued; // need this trait to call `value`

    #[test]
    fn test_storage() {
        let mut test_list = List::new();
        let rabel_bool = Bool::new(false);
        //let rabel_bool = false;
        test_list.push(rabel_bool);

        let rabel_null = Null::new();
        test_list.push(rabel_null);

        let test_string = String::from("Test string");
        let text = Text::new(&test_string);
        test_list.push(text);

        // List stores a list.
        let new_test_list = List::new();
        test_list.push(new_test_list);

        assert_eq!(test_list.len(), 4);
        assert_eq!(test_list.internal[0].type_name(), "Bool");
        assert_eq!(test_list.internal[1].type_name(), "Null");
        assert_eq!(test_list.internal[2].type_name(), "Text");
    }
}

// Insert internal types into a list
mod test_insert_internal_types {
    use super::*;
    use crate::{bool::Bool, null::Null, integer::Integer, double::Double};
    //use crate::marker::ScalarValued; // need this trait to call `value`

    #[test]
    fn test_insert() {
        let mut test_list = List::new();
        let rabel_bool = Bool::new(false);
        test_list.push(rabel_bool);
        let rabel_null = Null::new();
        test_list.push(rabel_null);
        assert_eq!(test_list.len(), 2);
        assert_eq!(test_list.internal[0].type_name(), "Bool");
        assert_eq!(test_list.internal[1].type_name(), "Null");

        // insert an integer at index 1
        let rable_integer = Integer::new(100);
        test_list.insert(1, rable_integer);

        assert_eq!(test_list.len(), 3);
        assert_eq!(test_list.internal[0].type_name(), "Bool");
        assert_eq!(test_list.internal[1].type_name(), "Integer");
        assert_eq!(test_list.internal[2].type_name(), "Null");

        // insert a boxed object at 2
        let rable_double = Double::new(0.04);
        test_list.insert_box(2, Box::new(rable_double));
        assert_eq!(test_list.len(), 4);
        assert_eq!(test_list.internal[0].type_name(), "Bool");
        assert_eq!(test_list.internal[1].type_name(), "Integer");
        assert_eq!(test_list.internal[2].type_name(), "Double");
        assert_eq!(test_list.internal[3].type_name(), "Null");
    }

    #[test]
    #[should_panic]
    fn test_insert_failure() {
        let mut test_list = List::new();
        let rabel_bool = Bool::new(false);
        test_list.push(rabel_bool);
        let rabel_null = Null::new();
        test_list.push(rabel_null);
        assert_eq!(test_list.len(), 2);
        assert_eq!(test_list.internal[0].type_name(), "Bool");
        assert_eq!(test_list.internal[1].type_name(), "Null");

        // Panic: length is 2, insert an integer at index 3!
        let rable_integer = Integer::new(100);
        test_list.insert(3, rable_integer);
    }
}

// Indexing a list by using usize integer.
mod test_list_indexing {
    use super::*;
    use crate::{bool::Bool, null::Null, text::Text, double::Double};
    use crate::marker::ScalarValued; // need this trait to call `value`

    #[test]
    fn test_mutable_indexing() {
        let mut test_list = List::new();
        let rabel_bool = Bool::new(false);
        test_list.push(rabel_bool);
        assert_eq!(test_list.internal[0].type_name(), "Bool");
        // push one more element
        let rabel_null = Null::new();
        test_list.push(rabel_null);
        assert_eq!(test_list.internal[0].type_name(), "Bool");
        assert_eq!(test_list.internal[1].type_name(), "Null");
        // replace 0th element by a Null
        let rabel_null_2 = Null::new();
        test_list[0] = Box::new(rabel_null_2);
        assert_eq!(test_list.internal[0].type_name(), "Null");
        // replace 1st element by a Double
        let rabel_double = Double::new(10.09);
        test_list[1] = Box::new(rabel_double);
        assert_eq!(test_list.internal[1].type_name(), "Double");
    }
    
    #[test]
    fn test_indexing_out_of_range() {
        let mut test_list = List::new();
        test_list.push(Bool::new(false));
        test_list.push(Null::new());
        test_list.push(Text::new("Happy day"));
        
        // Index out of range.
        let item = test_list.get_ref::<Text>(3);
        match item {
            Err(err_msg) => {println!("{}", err_msg.full_message())},
            _ => panic!("You are not supposed to see this")
        }
        
        // index out of range
        let item = test_list.get_mut_ref::<Text>(4);
        match item {
            Err(err_msg) => {println!("{}", err_msg.full_message())},
            _ => panic!("You are not supposed to see this")
        }
    }

    #[test]
    fn test_indexing() {
        let mut test_list = List::new();
        test_list.push(Bool::new(false));
        test_list.push(Null::new());
        // push a text
        let test_string = String::from("Test string");
        test_list.push(Text::new(&test_string));
        // List stores a list.
        test_list.push(List::new());
        assert_eq!(test_list.len(), 4);

        // 0th element, Bool
        assert_eq!(test_list[0].type_name(), "Bool");
        // cannot move out of list
        let item0 = &test_list[0];
        assert_eq!(item0.type_name(), "Bool");
        
        // 1th element, Null
        let item1 = &test_list[1];
        assert_eq!(item1.type_name(), "Null");
        // check exact type of `item1`.
        assert!( item1.as_any().is::<Null>() );
        //
        match item1.as_any().downcast_ref::<Null>() {
            Some(el) => assert_eq!(el.value(), 0),
            None => panic!("element isn't a Null!"),
        };
        
        // 2nd element, String
        let item2 = &test_list[2];
        assert_eq!(item2.type_name(), "Text");
        assert!(item2.as_any().is::<Text>());
        match item2.as_any().downcast_ref::<Text>() {
            Some(el) => assert_eq!(el.value(), "Test string"),
            None => panic!("element isn't a Text!"),
        };
    }
}

// List contains another container
mod test_list_with_dict {
    use super::*;
    use crate::dict::Dict;
    //use crate::{bool::Bool, null::Null, text::Text, dict::Dict,
    //            integer::Integer, double::Double};
    //use crate::marker::ScalarValued; // need this trait to call `value`

    #[test]
    fn test_with_subdict() {
        let mut test_list = List::new();
        let test_dict = Dict::new();
        test_list.push(test_dict);

        let el0 = &test_list[0];
        assert!(el0.as_any().is::<Dict>());
        match el0.as_any().downcast_ref::<Dict>() {
            Some(el) => assert_eq!(el.type_name(), "Dict"),
            None => panic!("element isn't a Dict!"),
        };
    }
}

// Methods due to `Container` trait.
mod test_list_container_trait_methods {
    use super::*;
    use crate::{bool::Bool, null::Null, text::Text, dict::Dict,
                integer::Integer, double::Double};
    //use crate::marker::ScalarValued; // need this trait to call `value`

    #[test]
    fn test_set() {
        let mut test_list = List::new();
        
        // Since list is empty, set/insert at index 0 is okay.
        match test_list.set(0, Text::new("Hello")) {
            Ok(_) => {},
            _ => panic!("You shouldn't see this!"),
        }
        
        // Since list has only 1 item, set/insert at index 2 is NOT allowed
        match test_list.set(2, Text::new("Word")) {
            Ok(_) => panic!("Insert index is out of range! Shall not be okay."),
            _ => {}
        }
    }

    #[test]
    fn test_set_from() {
        let mut test_list = List::new();
        // Since list is empty, set/insert at index 0 is okay.
        match test_list.set_from(0, "Hello") {
            Ok(_) => {},
            _ => panic!("You shouldn't see this!"),
        }
        // Failure! Since list has only 1 item, set/insert at index 2 is NOT allowed
        match test_list.set_from(2, "Word") {
            Ok(_) => panic!("Insert index is out of range! Shall not be okay."),
            _ => {}
        }
        match test_list.set_from(1, 100) {
            Ok(_) => {},
            _ => panic!("You shouldn't see this!"),
        }
        // Now it is okay.
        match test_list.set_from(2, 0.077) {
            Ok(_) => {},
            _ => panic!("You shouldn't see this!"),
        }
        assert_eq!(test_list.get_type_name(0).unwrap(), "Text");
        assert_eq!(test_list.get_type_name(1).unwrap(), "Integer");
        assert_eq!(test_list.get_type_name(2).unwrap(), "Double");
    }

    #[test]
    fn test_get_type_name() {
        // A mutable list
        let mut test_list = List::new();
        // push first item in
        let rabel_bool = Bool::new(false);
        test_list.push(rabel_bool);
        // push second item
        let sub_list = List::new();
        test_list.push(sub_list);
        assert_eq!(test_list.get_type_name(0).unwrap(), "Bool");
        assert_eq!(test_list.get_type_name(1).unwrap(), "List");
    }

    #[test]
    fn test_get_ref() {
        // A mutable list
        let mut test_list = List::new();
        // push first item in
        let rabel_bool = Bool::new(false);
        test_list.push(rabel_bool);
        // push second item
        let sub_list = List::new();
        test_list.push(sub_list);
        
        // Method get_ref must specify a type
        let el_bool  = test_list.get_ref::<Bool>(0).unwrap();
        assert_eq!(el_bool.type_name(), "Bool");

        // Access and modify second sub list
        let el_list = test_list.get_mut_ref::<List>(1).unwrap();
        let rabel_bool = Bool::new(true);
        el_list.push(rabel_bool);
        // now, sub list has one item
        assert_eq!(test_list.get_ref::<List>(1).unwrap().len(), 1);
    }

    #[test]
    fn test_get_ref_mismatched_type() {
        // A mutable list
        let mut test_list = List::new();
        // push first item in
        let rabel_bool = Bool::new(false);
        test_list.push(rabel_bool);
        // push second item
        let sub_list = List::new();
        test_list.push(sub_list);
        
        // Intend to return a Null type. But the target shall be Bool.
        // Must return Err.
        let el_bool  = test_list.get_ref::<Null>(0);
        match el_bool {
            Err(_) => {}, // should return Err.
            _ => panic!("You shouldn't be seeing this!")
        }

        // Intend to return Dict, but element is List
        // Must return Err, otherwise should panic
        match test_list.get_ref::<Dict>(1) {
            Err(_) => {}, // should return Err.
            _ => panic!("You shouldn't be seeing this!")
        }
    }

    // Test to return a mutable reference from a container and modify it.
    #[test]
    fn test_get_mut_ref() {
        // A mutable list
        let mut test_list = List::new();
        // push first item in
        let rabel_bool = Bool::new(false);
        test_list.push(rabel_bool);
        // push second item
        let mut sub_list = List::new();
        let integer = Integer::from(1000);
        sub_list.push(integer);
        test_list.push(sub_list);

        // Acquire a mutable reference to sub list.
        let sub_list_ref = test_list.get_mut_ref::<List>(1).unwrap();
        let number = Double::new(9.5);
        // push a new item into sub list.
        sub_list_ref.push(number);
        // sub list now has two items
        assert_eq!(sub_list_ref.len(), 2);

        // first item in the sub list is an Integer
        let first_item = sub_list_ref.get_ref::<Integer>(0).unwrap();
        assert_eq!(*first_item, 1000);
        // second item in the sub list is a Double
        let second_item = sub_list_ref.get_ref::<Double>(1).unwrap();
        assert_eq!(*second_item, 9.5);

        // Push another item into sub list.
        let text = Text::from("Hello, World");
        sub_list_ref.push(text);
        // now sublist has 3 item
        assert_eq!(sub_list_ref.len(), 3);
        // second item in the sub list is a Double
        let third_item = sub_list_ref.get_ref::<Text>(2).unwrap();
        assert_eq!(*third_item, "Hello, World");
    }
}