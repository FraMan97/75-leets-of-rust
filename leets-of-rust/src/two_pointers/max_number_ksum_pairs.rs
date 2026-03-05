// You are given an integer array nums and an integer k.

// In one operation, you can pick two numbers from the array whose sum equals k and remove them from the array.

// Return the maximum number of operations you can perform on the array.

use std::collections::HashMap;

pub fn max_number_ksum_pairs(nums: &[i32], k: &i32) -> i32 {
    let mut map = HashMap::new();
    let mut count = 0;

    for x in nums {
        let target = k - x;
        
        let entry = map.entry(target).or_insert(0);
        
        if *entry > 0 {
            count += 1;
            *entry -= 1;
        } else {
            *map.entry(*x).or_insert(0) += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::max_number_ksum_pairs;
    
    #[test]
    fn test_max_number_ksum_pairs(){
        let nums = [1,2,3,4];
        let k = 5;
        let result = max_number_ksum_pairs(&nums, &k);
        assert_eq!(result, 2);
    }

        
    #[test]
    fn test_empty_max_number_ksum_pairs(){
        let nums = [];
        let k = 5;
        let result = max_number_ksum_pairs(&nums, &k);
        assert_eq!(result, 0);
    }
}