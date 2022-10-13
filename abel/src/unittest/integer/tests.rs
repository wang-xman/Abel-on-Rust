//! unittest crate integer
use super::*;

mod test_integer_constructor {
    use super::*;

    #[test]
    fn test_constructor() {
        let int : Integer = Integer::new(100);
        let mut int2 : Integer = Integer::new(200);
        assert_eq!(int.value(), 100);
        assert_eq!(int2, 200);
        assert_eq!(int.type_name(), "Integer");
        int2 = int; // Caution! Move semantics
        assert_eq!(int2.value(), 100);
    }
}

mod test_integer_partial_eq {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let int : Integer = Integer::new(100);
        assert_eq!(int, 100);
        assert_eq!(100, int);
        // compare to double (f64)
        assert_eq!(int, 100.0);
        assert_eq!(100.0, int);
        let int2 : Integer = Integer::new(100);
        assert_eq!(int, int2);
    }
}