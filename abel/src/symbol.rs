/// Standalone symbols
pub const SPACE: char = ' ';  // Single space
pub const SHARP: char = '#';  // Single-line comment
pub const COLON: char = ':';  // Key-value separator
pub const COMMA: char = ',';  // Same level object separator
pub const ENDL: char =  '\n';  // End of line (not in active use)
pub const BACK_SLASH: char = '\\';  // Back slash, need escape
pub const FORWARD_SLASH: char = '/';  // Forward slash, needs no escape
pub const SINGLE_QUOTE: char = '\'';  // Single quote
pub const DOUBLE_QUOTE: char = '\"';  // Double quote
pub const BACK_TICK: char = '`'; // Back tick `
//pub const GRAVE_ACCENT { "\u0060" }; // Grave accent `

pub const MINUS: char = '-';  // Minus sign
pub const PLUS: char = '+';  // Plus sign
pub const DECIMAL_POINT: char = '.';  // Decimal point
pub const HAT: char = '^';  // Hat symbol, as exponentiation
pub const DOUBLE_MULTIPLY: &str = "**"; // Double multiply, as exponentiation

/// Standard arithmetic symbols
pub const ADD: char = '+';  // Add +
pub const SUBTRACT: char = '-';  // Subtract -
pub const MULTIPLY: char = '*';  // Multiply *
pub const DIVIDE: char = '/';  // Divide /

/// Paried symbols
/// Braces (curly brackets)
pub const L_BRACE: char = '{';
pub const R_BRACE: char = '}';
/// Brackets (square brackets)
pub const L_BRACKET: char = '[';
pub const R_BRACKET: char = ']';
/// Parentheses (round brackets)
pub const L_PARENTHESIS: char = '(';
pub const R_PARENTHESIS: char = ')';
/// Chevrons (angle brackets) 
pub const L_CHEVRON: char = '<';
pub const R_CHEVRON: char = '>';

/// Container symbols
pub const CONTAINER_TYPES: [&'static str; 3] = [ "Dict", "List", "Tuple" ];
pub const ITERABLE_CONTAINERS: [&'static str; 2]  = [ "List", "Tuple" ];
pub const CONTAINER_OPENING_SYMBOLS: [char; 3] = [
    L_BRACE, L_BRACKET, L_PARENTHESIS
];

pub const CONTAINER_CLOSING_SYMBOLS: [char; 3] = [
    R_BRACE, R_BRACKET, R_PARENTHESIS
];
pub const CONTAINER_OPENING_TOKEN_TYPES: [&'static str; 3] = [
    "DictOpening", "ListOpening", "TupleOpening"
];
pub const CONTAINER_CLOSING_TOKEN_TYPES: [&'static str; 3] = [
    "DictClosing", "ListClosing", "TupleClosing"
];