//! Unittest for crate bool 
//! Source file: bool.rs

use super::*;

mod test_bool {
    use super::*;

    #[test]
    fn test_constructor_from_var() {
        let rust_value = true;
        let rabel_bool = Bool::new(rust_value);
        assert_eq!( rust_value, rabel_bool.value() );
    }

    #[test]
    fn test_constructor_from_value() {
        let rabel_bool = Bool::new(false);
        assert_eq!( false, rabel_bool.value() );
    }

    #[test]
    fn test_from_string() {
        assert_eq!(false, Bool::from_str("false").unwrap().value());
        assert_eq!(true, Bool::from_str("true").unwrap().value());
        assert_eq!(Bool::from_str("TRUE").unwrap_err().get_header(),
                   "FAILED_TO_IDENTIFY");
    }

    #[test]
    fn test_partial_eq() {
        let rbool = Bool::new(true);
        let rbool2 = Bool::new(true);
        // Bool == bool
        assert_eq!(rbool, true);
        assert_ne!(rbool, false);
        // bool == Bool
        assert_eq!(true, rbool);
        assert_ne!(false, rbool2);
        // Bool == Bool
        assert_eq!(rbool, rbool2);
        assert_eq!(rbool2, rbool);
    }
}

mod test_bool_deref {
    use super::*;

    fn func(val: &bool, cmp: &bool) {
        assert_eq!(val, cmp);
    }

    #[test]
    fn test_deref() {
        let rtrue = Bool::new(true);
        let rfalse = Bool::new(false);
        let var_true = true;
        let var_false = false;
        func(&rtrue, &var_true);
        func(&rfalse, &var_false);
    }
}