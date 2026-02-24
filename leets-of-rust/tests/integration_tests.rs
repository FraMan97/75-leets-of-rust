use leets_of_rust::array_string::{gcd_of_strings::gcd_of_strings, kids_with_candies::kids_with_candies, merge_strings_alternately::merge_strings_alternately};



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

#[test]
fn test_kids_with_candies(){
    let candies = vec![2,3,5,1,3];
    let extra_candies = 3;
    let result = kids_with_candies(&candies, extra_candies);
    assert_eq!(result, vec![true, true, true, false, true]);
}