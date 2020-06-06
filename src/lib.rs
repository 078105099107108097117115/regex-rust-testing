#[macro_use] extern crate lazy_static;
use regex::Regex;

pub fn match_given_time_and_date(text: &str) -> bool {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"\d{4}-\d{2}-\d{2}[[:space:]]\d{2}:\d{2}[pa[m]]").unwrap();
    }
    RE.is_match(text)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn match_text_test() {
        let text = "This is the date and time today : 2020-06-06 12:30pm";
        assert!(match_given_time_and_date(text));
    }
}
