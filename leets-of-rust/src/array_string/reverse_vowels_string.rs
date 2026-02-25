// Given a string s, reverse only all the vowels in the string and return it.

// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.

pub fn reverse_vowels_string(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    
    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);

    let is_vowel = |c: char| -> bool {
        matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
    };

    while left < right {
        if !is_vowel(chars[left]) {
            left += 1;
            continue;
        }
        if !is_vowel(chars[right]) {
            right -= 1;
            continue;
        }

        chars.swap(left, right);
        left += 1;
        right -= 1;
    }

    *s = chars.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::reverse_vowels_string;

    #[test]
    fn test_reverse_vowels_string(){
        let mut s = String::from("IceCreAm");
        reverse_vowels_string(&mut s);
        assert_eq!(s, "AceCreIm");
    }

    #[test]
    fn test_empty_reverse_vowels_string(){
        let mut s = String::from("");
        reverse_vowels_string(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn test_no_reverse_vowels_string(){
        let mut s = String::from("qwrty");
        reverse_vowels_string(&mut s);
        assert_eq!(s, "qwrty");
    }
}