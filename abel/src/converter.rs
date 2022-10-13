use crate::error::{ErrorKind, Error, ParserError};
use crate::null::Null;
use crate::bool::Bool;
use crate::integer::Integer;
use crate::double::Double;
use crate::binary::Binary;
use crate::complex::Complex;
use crate::bitstring::Bitstring;
use crate::util;

pub fn is_null(src_str: &str) -> bool {
    src_str == "null"
}

pub fn as_null(src_str: &str) -> Result<Null, ParserError> {
    match is_null(src_str) {
        true => Ok(Null::new()),
        false => Err(ParserError::new(
            "Failed to identify and convert a Null type from string.",
            ErrorKind::FailedToIdentify
        )),
    }
}

pub fn is_bool(src_str: &str) -> bool {
    src_str == "true" || src_str == "false"
}

pub fn as_bool(src_str: &str) -> Result<Bool, ParserError> {
    match is_bool(src_str) {
        true => match src_str {
            "true" => Ok(Bool::new(true)),
            _ => Ok(Bool::new(false)),
        },
        false => Err(ParserError::new(
            "Failed to identify and convert a Bool type from string.",
            ErrorKind::FailedToIdentify
        ))
    }
}

pub fn is_integer(src_str: &str) -> bool {
    match src_str.parse::<i32>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn as_integer(src_str: &str) -> Result<Integer, ParserError> {
    match is_integer(src_str) {
        true => {
            let mut x : i32 = 0;
            if let Ok(value) = src_str.parse::<i32>() {
                x = value;
            }
            Ok(Integer::new(x))
        },
        false => Err(ParserError::new(
            "Failed to identify and convert an Integer type from string.",
            ErrorKind::FailedToIdentify
        ))
    }
}

pub fn is_double(src_str: &str) -> bool {
    match src_str.parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn as_double(src_str: &str) -> Result<Double, ParserError> {
    match is_double(src_str) {
        true => {
            let mut x : f64 = 0.0;
            if let Ok(value) = src_str.parse::<f64>() { x = value; };
            Ok(Double::new(x))
        },
        false => Err(ParserError::new(
            "Failed to identify and convert a Double type from string.",
            ErrorKind::FailedToIdentify
        )),
    }
}

pub fn is_binary(src_str: &str) -> bool {
    return util::is_valid_binary_string(src_str);
}

pub fn as_binary(src_str: &str) -> Result<Binary, ParserError> {
    match is_binary(src_str) {
        true => Ok(Binary::new(src_str)),
        false => Err(ParserError::new(
            "Failed to identify and convert a Binary type from string.",
            ErrorKind::FailedToIdentify
        )),
    }
}

pub fn is_bitstring(src_str: &str) -> bool {
    return util::is_valid_bitstring(src_str);
}

pub fn as_bitstring(src_str: &str) -> Result<Bitstring, ParserError> {
    match is_bitstring(src_str) {
        true => Ok(Bitstring::new(src_str)),
        false => Err(ParserError::new(
            "Failed to identify and convert a Bitstring type from string.",
            ErrorKind::FailedToIdentify
        )),
    }
}

pub struct DataTypeIdentifier {
    is_identified: bool,
    type_string: String,
    //data_string: String,
    real_string: String,
    imag_string: String
}

impl DataTypeIdentifier {
    pub fn new() -> Self {
        Self {
            is_identified: false,
            type_string: "".to_owned(),
            //data_string: "".to_owned(),
            real_string: "".to_owned(),
            imag_string: "".to_owned()
        }
    }
}

/// Complex identifier. The imaginary unit `j` is required to be
/// the last char in the string
pub fn identify_complex(src_str: &str) -> DataTypeIdentifier {
    let mut dti = DataTypeIdentifier::new();
    dti.type_string = String::from("Complex");

    let mut ret = true;
    let strlen = src_str.len();
    let mut position_pivot_sign: usize = 0;
    let mut has_pivot_sign: bool = false;
    let src_chars: Vec<char> = src_str.chars().collect();

    if src_str == "j" { // only 'j', okay
        ret = true;
        dti.is_identified = true;
        dti.imag_string = String::from("1.0");
        dti.real_string = String::from("0.0");
    } else if src_str == "+j" {
        ret = true;
        dti.is_identified = true;
        dti.imag_string = String::from("+1.0");
        dti.real_string = String::from("0.0");
    } else if src_str == "-j" {
        ret = true;
        dti.is_identified = true;
        dti.imag_string = String::from("-1.0");
        dti.real_string = String::from("0.0");
    } else if !(src_chars[strlen - 1] == 'j') {
        // if last symbol is not 'j', check if the whole string is a double
        if is_double(src_str) {
            ret = true;
            dti.is_identified = true;
            dti.real_string = src_str.to_owned();
        } else {
            ret = false;
        }
    } else { // Symbol 'j' at the end and has at least 2 characters.
        // Search for the pivot sign from one letter before last `j`.
        for index in (0..(strlen - 2)).rev() {
            //print!(" index is {} \n", index);
            if src_chars[index] == '-' || src_chars[index] == '+' {
                // sign is not the first symbol
                if index != 0 {
                    // Must be (digit, 'j', or '.') on right side,
                    // (digit or '.') on left side
                    if (src_chars[index + 1] == 'j' || src_chars[index + 1] == '.'
                            || src_chars[index + 1].is_digit(10))
                         && (src_chars[index - 1] == '.'
                             || src_chars[index - 1].is_digit(10))
                    {
                        //print!("Found pivot sign in the middle, at {} \n", index);
                        has_pivot_sign = true;
                        position_pivot_sign = index;
                        break;
                    } else {
                        continue;
                    }
                } else { // Sign is the first symbol, index = 0
                    // Rule out two signs in a row, as double cannot parse it.
                    if strlen >= 2 && (src_chars[index + 1] == '-'
                                       || src_chars[index + 1] == '+') 
                    {
                        ret = false;
                        break;
                    }
                    // if no pivot sign found, first one is.
                    if !has_pivot_sign {
                        has_pivot_sign = true;
                    }
                }
            }
        } // end for loop of pivot sign search.

        // Pivot sign found
        if has_pivot_sign {
            //print!("has pivot sign \n");
            // Rule out '-.j' or '+.j' sequence
            if src_chars[position_pivot_sign + 1] == '.'
                 && src_chars[position_pivot_sign + 2] == 'j' {
                //ret = false;
            } else {
                // Sign is the first symbol
                if position_pivot_sign == 0 {
                    if src_chars[1] == 'j' { // Expressions '-j' or '+j' are okay
                        ret = true;
                        dti.is_identified = true;
                        dti.real_string = String::from("0.0");
                        //dti.imag_string = src_str;
                        if src_chars[0] == '-' {
                            dti.imag_string = "-1.0".to_owned();    
                        } else {
                            dti.imag_string = "1.0".to_owned();
                        }
                    } else {
                        // Test symbols from (include) the sign to the one
                        // before symbol 'j'.
                        let subliteral = &src_str[0..strlen-1];
                        ret = is_double(subliteral);
                        if ret {
                            dti.is_identified = true;
                            dti.real_string = String::from("0.0");
                            dti.imag_string = String::from(subliteral);
                        }
                    }
                } else { // pivot sign in between, has then two sub-literals
                    // CAUTION! Second parameter in the slice is the ending
                    // index of the slice. Different from the C++ string method
                    // `substr()` where the second parameter is the total
                    // number of chars in the intended string.
                    let leftsub = &src_str[0..position_pivot_sign];
                    let rightsub = &src_str[position_pivot_sign..(strlen-1)];
                    // Identify both sub-strings.
                    ret = is_double(leftsub) && is_double(rightsub);
                    if ret {
                        dti.is_identified = true;
                        // If left sub in empty, pure imaginary, set real
                        // part to "0.0"
                        if leftsub == "" {
                            dti.real_string = String::from("0.0");
                        } else {
                            dti.real_string = String::from(leftsub);    
                        }
                        dti.imag_string = String::from(rightsub);
                    }
                }
            }
        } else { // no pivot sign
            // Note: case of only 'j' has been discussed at the beginning
            let sub = &src_str[0..strlen-1];
            ret = is_double(sub);
            if ret {
                dti.is_identified = true;
                dti.real_string = String::from("0.0");
                dti.imag_string = String::from(sub);
            }
        }
    }
    return dti;
}

pub fn is_complex(src_str: &str) -> bool {
    let dti: DataTypeIdentifier = identify_complex(src_str);
    dti.is_identified
}

pub fn as_complex(src_str: &str) -> Result<Complex, ParserError> {
    let dti: DataTypeIdentifier = identify_complex(src_str);
    if dti.is_identified {
        let real = dti.real_string.parse::<f64>().unwrap();
        let imag = dti.imag_string.parse::<f64>().unwrap();
        Ok(Complex::new(real, imag))
    } else {
        Err(ParserError::new(
            "Failed to identify and convert a Complex type from string.",
            ErrorKind::FailedToIdentify
        ))
    }
}

#[cfg(test)]
#[path = "./unittest/converter/tests.rs"]
mod tests;