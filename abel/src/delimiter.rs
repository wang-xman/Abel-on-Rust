use crate::util::{is_opening_symbol, is_closing_symbol,
        get_closing_symbol_by_opening};

pub struct DelimiterPair {
    opening_symbol: char,
    closing_symbol: char,
    position_opening: Vec<i32>, //vec![i32; 2], // [line, column]
    position_closing: Vec<i32>, //vec![i32; 2],
    match_counter: i32,
}

pub fn is_matched(pair: &DelimiterPair) -> bool {
    pair.match_counter == 0
        && pair.position_closing[0] != -999
            && pair.position_closing[1] != -999
}

pub struct DelimiterMatch {
    pair_register: Vec<DelimiterPair>,
    total_matched_pairs: i32
}

impl DelimiterMatch {
    pub fn new() -> Self {
        DelimiterMatch {
            pair_register: vec![],
            total_matched_pairs: 0
        }
    }

    fn init_pair(&mut self, opening_sym: char, line: i32, column: i32) {
        let pair = DelimiterPair {
            opening_symbol: opening_sym,
            closing_symbol: get_closing_symbol_by_opening(opening_sym).unwrap(),
            position_opening: vec![line, column],
            position_closing: vec![-999, -999],
            match_counter: 1
        };
        self.pair_register.push(pair);
    }

    pub fn match_symbol(&mut self, curstr: char, line: i32, column: i32) {
        if is_opening_symbol(curstr) {
            // Update counters of existing opening symbol
            for pair in &mut self.pair_register {
                // Pairs with the same opening symbol must be updated.
                // Matched pair needs NO update.
                if !is_matched(&pair) && pair.opening_symbol == curstr {
                    pair.match_counter += 1;
                }
            }
            // Initialise a delimiter pair object by opening symbol and store
            // it in register.
            self.init_pair(curstr, line, column);
        } else if is_closing_symbol(curstr) {
            for pair in &mut self.pair_register {
                // Pairs with the same closing symbol must be updated.
                if !is_matched(&pair) && pair.closing_symbol == curstr {
                    pair.match_counter -= 1;
                    // Match found. Store the position of closing symbol.
                    if pair.match_counter == 0 {
                        pair.position_closing = vec![line, column];
                        self.total_matched_pairs += 1;
                    }
                }   
            }
        }
    }

    fn scan(&mut self, target: &str) {
        let mut column = 0;
        for ch in target.chars() {
            self.match_symbol(ch, 0, column);
            column += 1;
        }
    }

    pub fn are_all_matched(&self) -> bool {
        self.pair_register.len() == (self.total_matched_pairs as usize)
    }

    pub fn number_of_matched_pairs(&self) -> i32 {
        self.total_matched_pairs
    }
}

#[cfg(test)]
#[path = "./unittest/delimiter/tests.rs"]
mod tests;