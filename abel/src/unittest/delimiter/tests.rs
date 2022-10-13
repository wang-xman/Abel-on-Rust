//! Unittest delimiter crate
//! Crate location /src/delimiter.rs
use super::*;

mod test_delimiter {
    use super::*;

    #[test]
    fn test_str_all_matched() {
        // all delimiters are matched
        let test_string = "{ab[ [cdr ]] {1hd} }";
        let mut matcher = DelimiterMatch::new();
        matcher.scan(test_string);
        assert!(matcher.are_all_matched());
    }

    #[test]
    fn test_str_not_all_matched() {
        // not all delimiters are matched
        let test_string = "{ab[ [cdr ] {1hd} }";
        let mut matcher = DelimiterMatch::new();
        matcher.scan(test_string);
        assert!(!matcher.are_all_matched());
        // only 3 matched pairs
        assert_eq!(matcher.number_of_matched_pairs(), 3);
    }

    #[test]
    fn test_string() {
        // all delimiters are matched
        let test_string = String::from("{ab[ [cdr ]] {1hd} }");
        let mut matcher = DelimiterMatch::new();
        matcher.scan(&test_string);
        assert!(matcher.are_all_matched());
    }
}
