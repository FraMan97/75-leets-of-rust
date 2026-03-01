// Given an array of characters chars, compress it using the following algorithm:

// Begin with an empty string s. For each group of consecutive repeating characters in chars:

// If the group's length is 1, append the character to s.
// Otherwise, append the character followed by the group's length.

// The compressed string s should not be returned separately, but instead, be stored in the input character array chars. Note that group lengths that are 10 or longer will be split into multiple characters in chars.

// After you are done modifying the input array, return the new length of the array.

// You must write an algorithm that uses only constant extra space.

//Note: The characters in the array beyond the returned length do not matter and should be ignored.

pub fn string_compression(chars: &mut Vec<char>) -> usize {
    let mut write = 0;
    let mut read = 0;

    while read < chars.len() {
        let current_char: char = chars[read];
        let mut count = 0;

        while read < chars.len() && chars[read] == current_char {
            read += 1;
            count += 1;
        }

        chars[write] = current_char;
        write += 1;

        if count > 1 {
            for c in count.to_string().chars() {
                chars[write] = c;
                write += 1;
            }
        }
    }

    write
}

#[cfg(test)]
mod tests {
    use super::string_compression;

    #[test]
    fn test_string_compression(){
        let mut nums = vec!['a','a','b','b','c','c','c'];
        let result = string_compression(&mut nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_empty_string_compression(){
        let mut nums = vec![];
        let result = string_compression(&mut nums);
        assert_eq!(result, 0);
    }
}