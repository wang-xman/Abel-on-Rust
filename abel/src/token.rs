pub trait Token {
    fn type_name(&self) -> &'static str;

    fn token_to_string(&self) -> String;
}