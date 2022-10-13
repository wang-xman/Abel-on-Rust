//! Unittest for null crate.

use super::*;

mod test_null {
    use super::*;

    #[test]
    fn test_constructor() {
        let null = Null::new();
        assert_eq!(null.value(), 0);
        assert_eq!(null.type_name(), "Null");
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Null::from_str("null").unwrap().value(), 0);
    }

    #[test]
    fn test_compare() {
        let n1 = Null::new();
        let n2 = Null::from_str("null").unwrap();
        assert_eq!(n1, n2);
    }
}