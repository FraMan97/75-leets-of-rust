// Given a binary array nums, you should delete one element from it.

// Return the size of the longest non-empty subarray containing only 1's in the resulting array. 
// Return 0 if there is no such subarray.
pub fn longest_subarray_1_after_deleting_one_element(nums: &[u8]) -> usize {
    let mut left = 0;
    let mut zeros = 0;
    let mut max_len = 0;

    for right in 0..nums.len() {
        if nums[right] == 0 {
            zeros += 1;
        }

        while zeros > 1 {
            if nums[left] == 0 {
                zeros -= 1;
            }
            left += 1;
        }

        max_len = max_len.max(right - left);
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::longest_subarray_1_after_deleting_one_element;

    #[test]
    fn test_longest_subarray_1_after_deleting_one_element(){
        let nums = [1,1,0,1];
        let result = longest_subarray_1_after_deleting_one_element(&nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_empty_longest_subarray_1_after_deleting_one_element(){
        let nums = [];
        let result = longest_subarray_1_after_deleting_one_element(&nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_all_1_longest_subarray_1_after_deleting_one_element(){
        let nums = [1,1,1,1];
        let result = longest_subarray_1_after_deleting_one_element(&nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_all_0_longest_subarray_1_after_deleting_one_element(){
        let nums = [0,0,0,0];
        let result = longest_subarray_1_after_deleting_one_element(&nums);
        assert_eq!(result, 0);
    }
}