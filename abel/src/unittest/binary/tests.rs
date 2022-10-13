//! Unittest binary crate
//! Crate location /src/binary.rs

use super::*;

mod test_binary_constructor {
    use super::*;

    #[test]
    #[should_panic]
    fn test_binary_constructor_new_panic() {
        let src = "19887312";
        Binary::new(src);

        let src = "0b";
        Binary::new(src);

        let src = "0ba";
        Binary::new(src);
    }

    #[test]
    #[should_panic]
    fn test_binary_constructor_from_panic() {
        let src = "19887312";
        Binary::from(src);

        let src = "0b";
        Binary::from(src);

        let src = "0ba";
        Binary::from(src);
    }

    #[test]
    fn test_binary_constructor_success() {
        let src = "0b1";
        Binary::from(src);

        let src = "0b0";
        Binary::from(src);

        let src = "0b0000000";
        Binary::from(src);
    }
}

mod test_binary_scalar_valued_trait {
    use super::*;

    #[test]
    fn test_value() {
        let src = "0b01010";
        let binary = Binary::from(src);
        assert_eq!(binary.value(), src);

        let src = "0b0";
        let binary = Binary::from(src);
        assert_eq!(binary.value(), src);

        let src = "0b1";
        let binary = Binary::from(src);
        assert_eq!(binary.value(), src);

        let src = "0b0000000";
        let binary = Binary::from(src);
        assert_eq!(binary.value(), src);
    }
}

mod test_binary_partial_eq {
    use super::*;

    #[test]
    fn test_eq() {
        let src = "0b01010";
        let b1 = Binary::from(src);
        let b2 = Binary::from("0b01010");

        assert_eq!(b1,b2);
    }
}