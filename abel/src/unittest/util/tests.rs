//! Unittest util(ity) crate
//! Crate location /src/util.rs

use super::*;

mod test_util_string_utils {
    use super::*;

    #[test]
    fn test_remove_space() {
        let original_str = "Hel lo wor ld";
        let result = remove_space(original_str);
        assert_eq!(result, "Helloworld");
        assert_ne!(result, original_str);

        let original_str = String::from("Hel lo wor ld");
        let result = remove_space(&original_str);
        assert_eq!(result, "Helloworld");
        assert_ne!(result, original_str);
    }

    #[test]
    fn test_has_letter() {
        // true
        let test_str = "Hel lo wor ld";
        assert_eq!(has_letter(test_str), true);
        
        // false
        let test_str = "198727.98";
        assert_eq!(has_letter(test_str), false);

        // false
        let test_str = "";
        assert_eq!(has_letter(test_str), false);

        // true
        let test_str = " 9ab0B";
        assert_eq!(has_letter(test_str), true);

        // true
        let test_str = " BAZFDFWD";
        assert_eq!(has_letter(test_str), true);

        // true
        let test_str = " a c b p   ";
        assert_eq!(has_letter(test_str), true);

        // true
        let test_str = " ABCDE FG XYZakl  nnihq web6";
        assert_eq!(has_letter(test_str), true);
    }

    #[test]
    fn test_begins_with_letter() {
        // false
        let test_str = " Hel lo wor ld";
        assert_eq!(begins_with_letter(test_str), false);
        
        // false
        let test_str = "198727.98";
        assert_eq!(begins_with_letter(test_str), false);

        let test_str = "a.98";
        assert_eq!(begins_with_letter(test_str), true);

        let test_str = "$1#.98";
        assert_eq!(begins_with_letter(test_str), false);
    }

    #[test]
    fn test_begins_with_sign() {
        // false
        let test_str = " Hel lo wor ld";
        assert_eq!(begins_with_sign(test_str), false);
        
        // false
        let test_str = "198727.98";
        assert_eq!(begins_with_sign(test_str), false);

        let test_str = "+ a.98";
        assert_eq!(begins_with_sign(test_str), true);

        let test_str = "-  $1#.98";
        assert_eq!(begins_with_sign(test_str), true);
        
        // leading space retuns false
        let test_str = " -  $1#.98";
        assert_eq!(begins_with_sign(test_str), false);
        let test_str = "   +1.98";
        assert_eq!(begins_with_sign(test_str), false);
    }

    #[test]
    fn test_is_algebraic_operator() {
        // true
        let test_str = "+";
        assert_eq!(is_algebraic_operator(test_str), true);
        let test_str = "-";
        assert_eq!(is_algebraic_operator(test_str), true);
        let test_str = "*";
        assert_eq!(is_algebraic_operator(test_str), true);
        let test_str = "/";
        assert_eq!(is_algebraic_operator(test_str), true);

        // false, due to leading space
        let test_str = " +";
        assert_eq!(is_algebraic_operator(test_str), false);
        let test_str = "  -";
        assert_eq!(is_algebraic_operator(test_str), false);
        let test_str = " *";
        assert_eq!(is_algebraic_operator(test_str), false);
        let test_str = " /";
        assert_eq!(is_algebraic_operator(test_str), false);

        let test_str = "$";
        assert_eq!(is_algebraic_operator(test_str), false);
        let test_str = "%";
        assert_eq!(is_algebraic_operator(test_str), false);
    }
}

mod test_util_is_valid_binary_string {
    use super::*;

    #[test]
    fn test_string() {
        let src = "01001"; // false
        assert!( !is_valid_binary_string(src) );

        let src = "12nc78"; // false
        assert!( !is_valid_binary_string(src) );

        let src = "0b078"; // false
        assert!( !is_valid_binary_string(src) );

        let src = "0b"; // false
        assert!( !is_valid_binary_string(src) );

        let src = "0b01 01"; // 
        assert!( !is_valid_binary_string(src) );

        let src = "0b0101"; // true
        assert!( is_valid_binary_string(src) );

        let src = "0b0"; // true
        assert!( is_valid_binary_string(src) );

        let src = "0b1"; // true
        assert!( is_valid_binary_string(src) );

        let src = "0b10000"; // true
        assert!( is_valid_binary_string(src) );

        let src = "0b11111"; // true
        assert!( is_valid_binary_string(src) );
        
    }
}

mod test_util_is_valid_bitstring {
    use super::*;

    #[test]
    fn test_string() {
        let src = "01001"; // false
        assert!( !is_valid_bitstring(src) );

        let src = "12nc78"; // false
        assert!( !is_valid_bitstring(src) );

        let src = "0b078"; // false
        assert!( !is_valid_bitstring(src) );

        let src = "b"; // false
        assert!( !is_valid_bitstring(src) );

        let src = "_b01 01"; // faslse
        assert!( !is_valid_bitstring(src) );

        let src = "_b."; // false
        assert!( !is_valid_bitstring(src) );

        let src = "_b0101"; // true
        assert!( is_valid_bitstring(src) );

        let src = "_b0.10.1"; // false, more than 1 dot
        assert!( !is_valid_bitstring(src) );

        let src = "_b0"; // true
        assert!( is_valid_bitstring(src) );

        let src = "_b1"; // true
        assert!( is_valid_bitstring(src) );

        let src = "_b10000"; // true
        assert!( is_valid_bitstring(src) );

        let src = "_b11111"; // true
        assert!( is_valid_bitstring(src) );
        
        let src = "_b.10000"; // true
        assert!( is_valid_bitstring(src) );

        let src = "_b1.1111"; // true
        assert!( is_valid_bitstring(src) );
    }
}

mod test_util_symbol_utils {
    use super::*;

    #[test]
    fn test_is_closing_symbol() {
        let sym = '}';
        assert!(is_closing_symbol(sym));

        let sym = ')';
        assert!(is_closing_symbol(sym));

        let sym = ']';
        assert!(is_closing_symbol(sym));
        
        // not closing symbols
        let sym = '(';
        assert!(!is_closing_symbol(sym));

        let sym = '{';
        assert!(!is_closing_symbol(sym));

        let sym = '[';
        assert!(!is_closing_symbol(sym));

        let sym = '9';
        assert!(!is_closing_symbol(sym));

        let sym = '|';
        assert!(!is_closing_symbol(sym));
    }

    #[test]
    fn test_is_opening_symbol() {
        // not opening symbols
        let sym = '}';
        assert!(!is_opening_symbol(sym));

        let sym = ')';
        assert!(!is_opening_symbol(sym));

        let sym = ']';
        assert!(!is_opening_symbol(sym));
        
        // opening symbols
        let sym = '(';
        assert!(is_opening_symbol(sym));

        let sym = '{';
        assert!(is_opening_symbol(sym));

        let sym = '[';
        assert!(is_opening_symbol(sym));

        // not opening symbols
        let sym = '9';
        assert!(!is_opening_symbol(sym));

        let sym = '|';
        assert!(!is_opening_symbol(sym));
    }

    #[test]
    fn test_iterable_container() {
        // dictionary is not iterable
        let sym = "Dict";
        assert!(!is_iterable_container(sym));

        let sym = "List";
        assert!(is_iterable_container(sym));

        let sym = "Tuple";
        assert!(is_iterable_container(sym));
        
        // don't know what this is.
        let sym = "Ahahab";
        assert!(!is_iterable_container(sym));
    }
}