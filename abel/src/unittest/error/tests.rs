//! Unittest for crate error
//! Source file: error.rs

use super::*;

mod test_error_kind {
    use super::*;

    #[test]
    fn test_enum_type() {
        let exception = ErrorKind::FileNotFound;
        assert_eq!(exception.get_enum_type(), "ErrorKind");
        let exception = ErrorKind::IncompatibleType;
        assert_eq!(exception.get_enum_type(), "ErrorKind");
    }

    #[test]
    fn test_variant_header() {
        let exception = ErrorKind::FileNotFound;
        assert_eq!(exception.get_header(), "FILE_NOT_FOUND");
        let exception = ErrorKind::IncompatibleType;
        assert_eq!(exception.get_header(), "INCOMPATIBLE_TYPE");
    }
}

#[cfg(test)]
mod test_internal_error {
    use super::*;

    #[test]
    fn test_contructor() {
        // from static string 
        let error = InternalError::new("FileNotFoundError.", ErrorKind::FileNotFound);
        assert_eq!(error.get_msg(), "FileNotFoundError.");

        // from static string variable
        let msg = "FileNotFoundError.";
        let error = InternalError::new(msg, ErrorKind::FileNotFound);
        assert_eq!(error.get_msg(), msg);
    }
}


#[cfg(test)]
mod test_parser_error {
    use super::*;

    #[test]
    fn test_constructor() {
        // from static string 
        let error = ParserError::new("FileNotFoundError.", ErrorKind::FileNotFound);
        assert_eq!(error.get_msg(), "FileNotFoundError.");

        // from static string variable
        let msg = "FileNotFoundError.";
        let error = ParserError::new(msg, ErrorKind::FileNotFound);
        assert_eq!(error.get_msg(), msg);

        // from String instance
        let msg = String::from("Not the right type.");
        let error = ParserError::new(&msg, ErrorKind::FileNotFound);
        assert_eq!(error.get_msg(), msg);
        assert_eq!(error.get_line(), -99);
    }
    
    #[test]
    fn test_get_line(){
        let mut error = ParserError::new("Testing FileNotFound.", ErrorKind::FileNotFound);
        error.set_line(10);
        assert_eq!(error.get_line(), 10);
    
    
        let mut error = ParserError::new("Testing IncompatibleType.", ErrorKind::IncompatibleType);
        error.set_line(90);
        assert_eq!(error.get_line(), 90);
    }
}