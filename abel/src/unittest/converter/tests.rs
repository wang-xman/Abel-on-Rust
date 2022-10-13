//! Unittest converter crate
//! Crate location /src/converter.rs
//! 
use super::*;
use crate::object::Object;
use crate::marker::ScalarValued;

mod test_null {
    use super::*;

    #[test]
    fn test_is_null_from_string_literal() {
        // not null
        let test_string = "Hello";
        assert!(!is_null(test_string));
        // not null
        let test_string = "Null";
        assert!(!is_null(test_string));
        // not null
        let test_string = "";
        assert!(!is_null(test_string));
        // okay
        let test_string = "null";
        assert!(is_null(test_string));
    }

    #[test]
    fn test_is_null_from_string_type() {
        // not null
        let test_string = &String::from("Hello");
        assert!(!is_null(test_string));
        // not null
        let test_string = &String::from("Null");
        assert!(!is_null(test_string));
        // not null
        let test_string = &String::from("");
        assert!(!is_null(test_string));
        // okay
        let test_string = &String::from("null");
        assert!(is_null(test_string));
    }

    #[test]
    fn test_as_null() {
        // okay
        let test_string = "null";
        if let Ok(res) = as_null(test_string) {
            assert_eq!(res.type_name(), "Null");
        }
        // error
        let test_string = "NULL";
        if let Err(err) = as_null(test_string) {
            assert_eq!(err.get_header(), "FAILED_TO_IDENTIFY");
        }

        // error
        let test_string = "Hello";
        if let Err(err) = as_null(test_string) {
            assert_eq!(err.get_header(), "FAILED_TO_IDENTIFY");
        }

        // error
        let test_string = " ";
        if let Err(err) = as_null(test_string) {
            assert_eq!(err.get_header(), "FAILED_TO_IDENTIFY");
        }
    }
}

#[cfg(test)]
mod test_integer {
    use super::*;

    #[test]
    fn test_is_integer() {
        // okay
        let test_string = "90";
        assert!(is_integer(test_string));

        // okay
        let test_string = "0";
        assert!(is_integer(test_string));

        // okay
        let test_string = "00";
        assert!(is_integer(test_string));

        // okay
        let test_string = "01";
        assert!(is_integer(test_string));

        // okay with sign
        let test_string = "+10";
        assert!(is_integer(test_string));

        // okay with sign
        let test_string = "-0";
        assert!(is_integer(test_string));

        // no
        let test_string = "+";
        assert!(!is_integer(test_string));
        // no
        let test_string = "-";
        assert!(!is_integer(test_string));

        // no
        let test_string = "9.0";
        assert!(!is_integer(test_string));

        // no
        let test_string = "0.1";
        assert!(!is_integer(test_string));
    }

    #[test]
    fn test_as_integer() {
        let test_string = "-90";
        if let Ok(int_obj) = as_integer(test_string) {
            assert!(int_obj.value() < 0);
            assert_eq!(int_obj.value(), -90);
        }

        let test_string = "+1";
        if let Ok(int_obj) = as_integer(test_string) {
            assert_eq!(int_obj.value(), 1);
        }

        // error
        let test_string = "0.05";
        //assert_eq!(as_null(test_string).unwrap().get_header(), Err(exceptions::Error));
        if let Err(err) = as_null(test_string) {
            assert_eq!(err.get_header(), "FAILED_TO_IDENTIFY");
        }

        let test_string = "90.0e+5";
        //assert_eq!(as_null(test_string),
        //           Err(exceptions::ErrorKind::DATA_TYPE_CONVERSION_ERROR));
        if let Err(err) = as_null(test_string) {
            assert_eq!(err.get_header(), "FAILED_TO_IDENTIFY");
        }
    }
}

#[cfg(test)]
mod test_double {
    use super::*;

    #[test]
    fn test_is_double() {
        // okay: integer is double
        let test_string = "90";
        assert!(is_double(test_string));
        // okay
        let test_string = "0";
        assert!(is_double(test_string));
        // okay
        let test_string = "00";
        assert!(is_double(test_string));

        // okay. regular double
        let test_string = ".0";
        assert!(is_double(test_string));
        let test_string = ".000";
        assert!(is_double(test_string));
        let test_string = "1.000";
        assert!(is_double(test_string));
        let test_string = "7.0";
        assert!(is_double(test_string));

        // scientific
        let test_string = "1.e-100";
        assert!(is_double(test_string));
        let test_string = ".00001e+10";
        assert!(is_double(test_string));
        let test_string = "500e-1";
        assert!(is_double(test_string));
        // scientific without sign
        let test_string = "500e10";
        assert!(is_double(test_string));
        let test_string = ".10010e9";
        assert!(is_double(test_string));
    }

    #[test]
    fn test_is_double_negative() {
        let test_string = "..e-100";
        assert!(!is_double(test_string));
        let test_string = "e";
        assert!(!is_double(test_string));
        let test_string = "1.0e";
        assert!(!is_double(test_string));
        let test_string = "10a";
        assert!(!is_double(test_string));
        let test_string = "1  .0";
        assert!(!is_double(test_string));
        let test_string = "  0.1";
        assert!(!is_double(test_string));
        let test_string = "e+";
        assert!(!is_double(test_string));
        let test_string = "+e";
        assert!(!is_double(test_string));
        let test_string = "e-";
        assert!(!is_double(test_string));
        let test_string = "-e";
        assert!(!is_double(test_string));
        // no
        let test_string = "1.0+0.5j";
        assert!(!is_double(test_string));
    }

    #[test]
    fn test_as_double() {
        let test_string = "-90";
        if let Ok(int_obj) = as_double(test_string) {
            assert!(int_obj.value() < 0.0);
            assert_eq!(int_obj.value(), -90.0);
        }

        let test_string = "+1";
        if let Ok(int_obj) = as_double(test_string) {
            assert_eq!(int_obj.value(), 1.0);
        }

        // error
        let test_string = "ab";
        if let Err(err) = as_null(test_string) {
            assert_eq!(err.get_header(), "FAILED_TO_IDENTIFY");
        }
        // error
        let test_string = ".0.e+5";
        if let Err(err) = as_null(test_string) {
            assert_eq!(err.get_header(), "FAILED_TO_IDENTIFY");
        }
    }
}

mod test_complex {
    use super::*;
    
    /*
    #[test]
    fn test_is_complex() {
        
        let test_string = "1.0j"; // true
        assert!( is_complex(test_string) );

        let test_string = "j"; //true
        assert!( is_complex(test_string) );

        let test_string = "0.5j"; //true
        assert!( is_complex(test_string) );

        let test_string = "0.0j"; //true
        assert!( is_complex(test_string) );

        let test_string = "1j"; //true
        assert!( is_complex(test_string) );

        let test_string = "0.5e-10j"; //true
        assert!( is_complex(test_string) );

        let test_string = "+0.5e-10j"; //true
        assert!( is_complex(test_string) );

        let test_string = "-0.5e+90j"; //true
        assert!( is_complex(test_string) );

        let test_string = "1.0+0.5e-10j"; // true
        assert!( is_complex(test_string) );

        let test_string = "-.5e+10+0.5e-10j"; // true
        assert!( is_complex(test_string) );
    }
    */

    #[test]
    fn test_complex_identifier() {
        
        let test_string = "1.0j";
        let dti: DataTypeIdentifier = identify_complex(test_string);
        assert_eq!(dti.imag_string, "1.0");

        let test_string = "j";
        let dti: DataTypeIdentifier = identify_complex(test_string);
        assert_eq!(dti.imag_string, "1.0");

        let test_string = "-1.j";
        let dti: DataTypeIdentifier = identify_complex(test_string);
        assert_eq!(dti.imag_string, "-1.");

        let test_string = "-0.5j";
        let dti: DataTypeIdentifier = identify_complex(test_string);
        assert_eq!(dti.real_string, "0.0");
        assert_eq!(dti.imag_string, "-0.5");
        
        let test_string = "-0.5e10j";
        let dti: DataTypeIdentifier = identify_complex(test_string);
        assert_eq!(dti.real_string, "0.0");
        assert_eq!(dti.imag_string, "-0.5e10");

        let test_string = "+0.05e-90-0.5e10j";
        let dti: DataTypeIdentifier = identify_complex(test_string);
        assert_eq!(dti.real_string, "+0.05e-90");
        assert_eq!(dti.imag_string, "-0.5e10");

        let test_string = ".05e+9+0.5e10j";
        let dti: DataTypeIdentifier = identify_complex(test_string);
        assert_eq!(dti.real_string, ".05e+9");
        assert_eq!(dti.imag_string, "+0.5e10");
    }

    #[test]
    fn test_as_complex() {
        let test_string = ".05e+9+0.5e10j";
        let complex = as_complex(test_string).unwrap();
        assert_eq!(complex.real(), 0.05e+9);
        assert_eq!(complex.imag(), 0.5e+10);

        let test_string = ".05j";
        let complex = as_complex(test_string).unwrap();
        assert_eq!(complex.real(), 0.0);
        assert_eq!(complex.imag(), 0.05);

        let test_string = "j";
        let complex = as_complex(test_string).unwrap();
        assert_eq!(complex.real(), 0.0);
        assert_eq!(complex.imag(), 1.0);

        let test_string = "-j";
        let complex = as_complex(test_string).unwrap();
        assert_eq!(complex.real(), 0.0);
        assert_eq!(complex.imag(), -1.0);

        let test_string = "-.005j";
        let complex = as_complex(test_string).unwrap();
        assert_eq!(complex.real(), 0.0);
        assert_eq!(complex.imag(), -0.005);

        let test_string = ".0-.0j";
        let complex = as_complex(test_string).unwrap();
        assert_eq!(complex.real(), 0.0);
        assert_eq!(complex.imag(), 0.0);
    }

}