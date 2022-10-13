//! Unittest bitstring crate
//! Crate location /src/bitstring.rs

use super::*;

mod test_bitstring_constructor {
    use super::*;

    #[test]
    #[should_panic]
    fn test_bitstring_constructor_new_panic() {
        let src = "19887312";
        Bitstring::new(src);

        let src = "0b";
        Bitstring::new(src);

        let src = "0ba";
        Bitstring::new(src);
    }

    #[test]
    #[should_panic]
    fn test_bitstring_constructor_panic_1() {
        let src = "_b19887312";
        Bitstring::from(src);

        let src = "_b0b";
        Bitstring::from(src);

        let src = "_b0ba";
        Bitstring::from(src);

        let src = "_b0ba";
        Bitstring::from(src);
    }

    #[test]
    #[should_panic]
    fn test_bitstring_constructor_panic_2() {
        let src = "_b1.0.0";
        Bitstring::from(src);
    }

    #[test]
    #[should_panic]
    fn test_bitstring_constructor_panic_3() {
        let src = "_b.100a";
        Bitstring::from(src);
    }

    #[test]
    fn test_bitstring_constructor_success() {
        let src = "_b1";
        Bitstring::from(src);

        let src = "_b0";
        Bitstring::from(src);

        let src = "_b0000000";
        Bitstring::from(src);
    }
}

mod test_bitstring_scalar_valued_trait {
    use super::*;

    #[test]
    fn test_value() {
        let src = "_b01010";
        let bs = Bitstring::from(src);
        assert_eq!(bs.value(), src);

        let src = "_b0";
        let bs = Bitstring::from(src);
        assert_eq!(bs.value(), src);

        let src = "_b.1";
        let bs = Bitstring::from(src);
        assert_eq!(bs.value(), src);

        let src = "_b0.000000";
        let bs = Bitstring::from(src);
        assert_eq!(bs.value(), src);
    }
}

mod test_bitstring_partial_eq {
    use super::*;

    #[test]
    fn test_eq() {
        let src = "_b010.10";
        let b1 = Bitstring::from(src);
        let b2 = Bitstring::from("_b010.10");

        assert_eq!(b1,b2);
    }
}