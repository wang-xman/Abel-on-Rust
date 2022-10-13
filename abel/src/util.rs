use crate::error;
use crate::symbol;

pub fn remove_space(src: &str) -> String {
    src.chars().filter(|ch| !ch.is_whitespace()).collect()
}

pub fn has_letter(src: &str) -> bool {
    src.chars().any(|ch| ch.is_alphabetic())
}

pub fn begins_with_letter(src: &str) -> bool {
    src.chars().next().unwrap().is_alphabetic()
}

pub fn begins_with_sign(src: &str) -> bool {
    let first_char = src.chars().next().unwrap();
    (first_char == '-') || (first_char == '+')
}

pub fn is_valid_binary_string(src: &str) -> bool {
    if src.len() <= 2 { // if only two characters, false
        false
    } else {
        if !(src.chars().next().unwrap() == '0'
             && src.chars().nth(1).unwrap() == 'b')
        { // if not begins with "0b", false
            false
        } else { // rest must be either '0' or '1'.
            !src[2..].chars().any(|ch| !(ch == '0' || ch == '1'))
        }
    }
}

pub fn is_valid_bitstring(src: &str) -> bool {
    let mut ret = true;
    let strlen = src.len();
    let mut dot_counter = 0;
    if strlen < 3 { // cannot be empty
        false
    } else if !(src.chars().next().unwrap() == '_'
                && src.chars().nth(1).unwrap() == 'b')
    { // must begin with "_b" prefix
        false
    } else if src == "_b." { // cannot be "_b.", makes no sense.
        false
    } else {  // rest must be either '0', '1', or dot
        //let ret = false;
        for (_, item) in src[2..].chars().enumerate() {
            if !(item == '0' || item == '1' || item == '.') {
                ret = false;
                break;
            } else if item == '.' {
                dot_counter += 1;
                if dot_counter > 1 { // more than 1 dot, no
                    ret = false;
                    break;
                } else { // THINK! Not needed as len must be >= 3
                    if strlen == 1 { // just a dot, no
                        ret = false;
                        break;
                    }
                }
            }
        }
        return ret;
    }
}

pub fn is_algebraic_operator(src_str: &str) -> bool {
    src_str == symbol::ADD.to_string()
    || src_str == symbol::SUBTRACT.to_string()
    || src_str == symbol::MULTIPLY.to_string()
    || src_str == symbol::DIVIDE.to_string()
    || src_str == symbol::HAT.to_string()
    || src_str == "**"
}

pub fn is_opening_symbol(sym_char: char) -> bool {
    symbol::CONTAINER_OPENING_SYMBOLS.iter().any(|&item| sym_char == item)
}

pub fn is_closing_symbol(sym_char: char) -> bool {
    symbol::CONTAINER_CLOSING_SYMBOLS.iter().any(|&item| sym_char == item)
}

pub fn is_iterable_container(type_str: &str) -> bool {
    symbol::ITERABLE_CONTAINERS.iter().any(|&item| item == type_str)
}

pub fn get_closing_symbol_by_opening(opening_sym: char)
-> Result<char, error::ErrorKind>
{
    match opening_sym {
        symbol::L_BRACE => Ok(symbol::R_BRACE),
        symbol::L_BRACKET => Ok(symbol::R_BRACKET),
        symbol::L_PARENTHESIS => Ok(symbol::R_PARENTHESIS),
        _ => Err(error::ErrorKind::UnrecognizedSymbol),
    }
}

#[cfg(test)]
#[path = "./unittest/util/tests.rs"]
mod tests;