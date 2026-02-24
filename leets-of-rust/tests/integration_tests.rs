use leets_of_rust::array_string::merge_strings_alternately::merge_strings_alternately;



#[test]
fn test_merge_strings_alternately() {
    let word1: String = String::from("abc");
    let word2: String = String::from("def");
    let result: String = merge_strings_alternately(&word1, &word2);
    assert_eq!(result, "adbecf");
}