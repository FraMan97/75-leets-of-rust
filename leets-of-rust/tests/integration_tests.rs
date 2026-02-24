use leets_of_rust::array_string::{gcd_of_strings::gcd_of_strings, merge_strings_alternately::merge_strings_alternately};



#[test]
fn test_merge_strings_alternately() {
    let word1: String = String::from("abc");
    let word2: String = String::from("def");
    let result: String = merge_strings_alternately(&word1, &word2);
    assert_eq!(result, "adbecf");
}

#[test]
fn test_gcd_of_strings() {
    let s: String = String::from("ABCABC");
    let t: String = String::from("ABC");
    let result: &str  = gcd_of_strings(&s, &t);
    assert_eq!(result, "ABC");
}