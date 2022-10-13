//! unittest crate text
use super::*;

mod test_text_constructor {
    use super::*;

    #[test]
    fn test_constructor() {
        let test_string = String::from("Test String");
        let rstring = Text::new(&test_string);
        assert_eq!(test_string, "Test String");
        assert_eq!(rstring.type_name(), "Text");
        assert_eq!(rstring.value(), test_string);

        let test_string = "Test literal";
        let rstring = Text::new(test_string);
        assert_eq!(test_string, "Test literal");
        assert_eq!(rstring.type_name(), "Text");
        assert_eq!(rstring.value(), test_string);
    }
}

mod test_text_value {
    use super::*;

    #[test]
    fn test_value() {
        let test_string = String::from("Test String");
        let text = Text::from(&test_string);
        // cloned a String and assigned to the left
        let text_value = text.value();
        // instance `text` is still valid
        assert_eq!(text.value(), text_value);
    }
}

mod test_text_partial_eq {
    use super::*;

    #[test]
    fn test_partial_eq_text_and_string() {
        // Compare Text and String
        let test_string = String::from("Test String");
        let text = Text::from(&test_string);
        // Text == String
        assert_eq!(text, test_string);
        // Text == &String
        assert_eq!(text, &test_string);
        // String == Text
        assert_eq!(test_string, text);
    }
    
    #[test]
        fn test_partial_eq_text_and_str() {    
        // Compare Text and &str
        let test_string = "Another String";
        let text = Text::from(test_string);
        assert_eq!(text, test_string);
        assert_eq!(test_string, text);
    }
}

mod test_text_deref {
    use super::*;

    // returns the length of a str
    fn func_str(text: &str) -> usize {
        text.len()
    }

    // returns the length of a &String
    fn func_string(text: &String) -> usize {
        text.len()
    }

    #[test]
    fn test_deref() {
        let text = Text::new("Hello");
        assert_eq!(5, func_str(&text));
        assert_eq!(5, func_string(&text));
    }
}