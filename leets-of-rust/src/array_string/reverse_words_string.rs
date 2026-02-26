// Given an input string s, reverse the order of the words.

// A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.

// Return a string of the words in reverse order concatenated by a single space.

// Note that s may contain leading or trailing spaces or multiple spaces between two words. 
// The returned string should only have a single space separating the words. Do not include any extra spaces.

pub fn reverse_words_in_string(s: &String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut result = String::new();
    
    if n == 0 { return result; }

    let mut start_word = false;
    let mut end_index_word = 0;

    for index in (0..n).rev() {
        if chars[index] != ' ' && !start_word {
            start_word = true;
            end_index_word = index; 
        }
        
        if start_word && (chars[index] == ' ' || index == 0) {
            let start = if chars[index] == ' ' { index + 1 } else { index };
            
            if !result.is_empty() {
                result.push(' ');
            }
            
            result.extend(&chars[start..end_index_word + 1]);
            start_word = false;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::reverse_words_in_string;

    #[test]
    fn test_reverse_words_in_string(){
        let s: String = String::from("the sky is blue");
        let result = reverse_words_in_string(&s);
        assert_eq!(result, "blue is sky the");
    }

    #[test]
    fn test_empty_reverse_words_in_string(){
        let s: String = String::from("");
        let result = reverse_words_in_string(&s);
        assert_eq!(result, "");
    }
}