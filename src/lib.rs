#[macro_use] extern crate lazy_static;
use regex::Regex;

pub fn match_text(text: &str) -> bool {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    }
    RE.is_match(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn match_text_test() {
        let text = "This is the date today : 2020-06-06";
        assert!(match_text(text));
    }
}
