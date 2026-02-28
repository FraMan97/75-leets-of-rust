// Given an integer array nums, return true if there exists a triple of indices (i, j, k) such 
// that i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.

pub fn increasing_triplet_subsequence(nums: &[i32]) -> bool {
    let mut first = i32::MAX;
    let mut second = i32::MAX;

    for &value in nums {
        if value <= first {
            first = value;
        } else if value <= second {
            second = value;
        } else {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::increasing_triplet_subsequence;

    #[test]
    fn test_no_triplets_incresing_triplet_subsequence(){
        let nums = vec![5,4,3,2,1];
        let result = increasing_triplet_subsequence(&nums);
        assert_eq!(result, false);
    }

    #[test]
    fn test_incresing_triplet_subsequence(){
        let nums = vec![2,1,5,0,4,6];
        let result = increasing_triplet_subsequence(&nums);
        assert_eq!(result, true);
    }
}