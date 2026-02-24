// You are given two strings word1 and word2. 
// Merge the strings by adding letters in alternating order, starting with word1. 
// If a string is longer than the other, append the additional letters onto the end of the merged string.

// Return the merged string.

pub fn merge_strings_alternately(word1: &str, word2: &str) -> String {
    let mut result = String::with_capacity(word1.len() + word2.len());
    
    let mut chars1 = word1.chars();
    let mut chars2 = word2.chars();

    loop {
        match (chars1.next(), chars2.next()) {
            (Some(c1), Some(c2)) => {
                result.push(c1);
                result.push(c2);
            }
            (Some(c1), None) => {
                result.push(c1);
                result.extend(chars1);
                break;
            }
            (None, Some(c2)) => {
                result.push(c2);
                result.extend(chars2);
                break;
            }
            (None, None) => break,
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::merge_strings_alternately;

    #[test]
    fn test_strings_equal_length(){
        let word1: String = String::from("abc");
        let word2: String = String::from("def");
        let result = merge_strings_alternately(&word1, &word2);
        assert_eq!(result, "adbecf");
    }

    #[test]
    fn test_word1_shorter_than_word2(){
        let word1: String = String::from("abc");
        let word2: String = String::from("defg");
        let result = merge_strings_alternately(&word1, &word2);
        assert_eq!(result, "adbecfg");
    }
    
    #[test]
    fn test_word2_shorter_than_word1(){
        let word1: String = String::from("abcd");
        let word2: String = String::from("efg");
        let result = merge_strings_alternately(&word1, &word2);
        assert_eq!(result, "aebfcgd");
    }
}