use regex_101::match_given_time_and_date;
fn main() {
    let y:usize = 10;
    let text = "Today's date is 2020-06-06 18:39pm";
    println!("{}", match_given_time_and_date(text));
    
}
