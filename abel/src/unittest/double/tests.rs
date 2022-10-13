//! unittest crate double
use super::*;

mod test_double_constructor {
    use super::*;

    #[test]
    fn test_constructor() {
        let test_value : Double = Double::new(0.09);
        assert_eq!(test_value.value(), 0.09);
        assert_eq!(test_value.type_name(), "Double");
    }
}

mod test_double_partial_eq {
    use super::*;
    #[test]
    fn test_partial_eq() {
        // Double == f64
        let double_value = Double::new(1.09);
        assert_eq!(1.09, double_value);
        assert_eq!(double_value, 1.09);
        
        // Double == i32
        let double_value = Double::new(200.0);
        assert_eq!(200, double_value);
        assert_eq!(double_value, 200);

        // Double == Integer
        let double_value = Double::new(200.0);
        let integer_value = Integer::new(200);
        assert_eq!(double_value, integer_value);
        assert_eq!(integer_value, double_value);
    }
}