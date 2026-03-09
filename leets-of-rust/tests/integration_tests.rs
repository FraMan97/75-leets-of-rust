use leets_of_rust::{array_string::{can_place_flowers::can_place_flowers, gcd_of_strings::gcd_of_strings, increasing_triplet_subsequence::increasing_triplet_subsequence, kids_with_candies::kids_with_candies, merge_strings_alternately::merge_strings_alternately, product_array_except_self::product_array_except_self, reverse_vowels_string::reverse_vowels_string, reverse_words_string::reverse_words_in_string, string_compression::string_compression}, sliding_window::{maximum_average_subarray::maximum_average_subarray, maximum_consecutive_ones::maximum_consecutive_ones, maximum_number_vowels_substring::maximum_number_vowels_substring}, two_pointers::{container_with_most_water::container_with_most_water, is_subsequence::is_subsequence, max_number_ksum_pairs::max_number_ksum_pairs, move_zeros::move_zeros}};



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

#[test]
fn test_can_place_flowers(){
    let flowerbed = vec![1,0,0,0,1];
    let n = 1;
    let result = can_place_flowers(&flowerbed, n);
    assert_eq!(result, true);
}

#[test]
fn test_reverse_vowels_string(){
    let mut s = String::from("IceCreAm");
    reverse_vowels_string(&mut s);
    assert_eq!(s, "AceCreIm");
}

#[test]
fn test_reverse_words_in_string(){
    let s: String = String::from("the sky is blue");
    let result = reverse_words_in_string(&s);
    assert_eq!(result, "blue is sky the");
}


#[test]
fn test_product_array_except_self(){
    let nums = vec![1,2,3,4];
    let result = product_array_except_self(&nums);
    assert_eq!(result, vec![24,12,8,6]);
}

#[test]
fn test_incresing_triplet_subsequence(){
    let nums = vec![2,1,5,0,4,6];
    let result = increasing_triplet_subsequence(&nums);
    assert_eq!(result, true);
}

#[test]
fn test_string_compression(){
    let mut nums = vec!['a','a','b','b','c','c','c'];
    let result = string_compression(&mut nums);
    assert_eq!(result, 6);
}

#[test]
fn test_move_zeros(){
    let mut nums = vec![0,1,0,3,12];
    move_zeros(&mut nums);
    assert_eq!(nums, [1,3,12,0,0]);

}

#[test]
fn test_is_subsequence(){
    let s = "abc";
    let t = "ahbgdc";
    let result = is_subsequence(&s, &t);
    assert_eq!(result, true);
}

#[test]
fn test_container_with_most_water() {
    let height = [1,8,6,2,5,4,8,3,7];
    let result = container_with_most_water(&height);
    assert_eq!(result, 49);
}
    
#[test]
fn test_max_number_ksum_pairs(){
    let nums = [1,2,3,4];
    let k = 5;
    let result = max_number_ksum_pairs(&nums, &k);
    assert_eq!(result, 2);
}

#[test]
fn test_maximum_average_subarray(){
    let nums = [1,12,-5,-6,50,3];
    let k: usize = 4;
    let result = maximum_average_subarray(&nums, k);
    assert_eq!(result, 12.75000);
}

#[test]
fn test_maximum_number_vowels_substring(){
    let nums = "abciiidef";
    let k: usize = 3;
    let result = maximum_number_vowels_substring(&nums, k);
    assert_eq!(result, 3);
}

#[test]
fn test_maximum_consecutive_ones(){
    let nums = [1,1,1,0,0,0,1,1,1,1,0];
    let k: usize = 2;
    let result = maximum_consecutive_ones(&nums, k);
    assert_eq!(result, 6);
}