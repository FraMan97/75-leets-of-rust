// Given a string s and an integer k, return the maximum number of vowel letters in any substring of s with length k.

// Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.

pub fn maximum_number_vowels_substring(s: &str, k: usize) -> usize {
    let bytes = s.as_bytes();
    if bytes.len() < k {
        return 0;
    }
    let is_vowel = |b: u8| -> bool {
        matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')
    };

    let mut current_vowels = 0;
    
    for i in 0..k {
        if is_vowel(bytes[i]) {
            current_vowels += 1;
        }
    }

    let mut max_vowels = current_vowels;

    for i in k..bytes.len() {
        if is_vowel(bytes[i]) {
            current_vowels += 1;
        }
        if is_vowel(bytes[i - k]) {
            current_vowels -= 1;
        }
        
        max_vowels = max_vowels.max(current_vowels);
        
        if max_vowels == k { return k; }
    }

    max_vowels
}


#[cfg(test)]
mod tests {
    use super::maximum_number_vowels_substring;

    #[test]
    fn test_maximum_number_vowels_substring(){
        let nums = "abciiidef";
        let k: usize = 3;
        let result = maximum_number_vowels_substring(&nums, k);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_empty_maximum_number_vowels_substring(){
        let nums = "";
        let k: usize = 4;
        let result = maximum_number_vowels_substring(&nums, k);
        assert_eq!(result, 0);
    }
}