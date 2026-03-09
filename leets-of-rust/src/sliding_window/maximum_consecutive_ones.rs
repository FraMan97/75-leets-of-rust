// Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the 
// array if you can flip at most k 0's.
pub fn maximum_consecutive_ones(nums: &[u8], k: usize) -> usize {
    let mut start = 0;
    let mut zeroes = 0;
    let mut max_len = 0;

    for end in 0..nums.len() {
        if nums[end] == 0 {
            zeroes += 1;
        }

        while zeroes > k {
            if nums[start] == 0 {
                zeroes -= 1;
            }
            start += 1;
        }

        max_len = max_len.max(end - start + 1);
    }

    max_len
}


#[cfg(test)]
mod tests {
    use super::maximum_consecutive_ones;

    #[test]
    fn test_maximum_consecutive_ones(){
        let nums = [1,1,1,0,0,0,1,1,1,1,0];
        let k: usize = 2;
        let result = maximum_consecutive_ones(&nums, k);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_empty_maximum_consecutive_ones(){
        let nums = [];
        let k: usize = 4;
        let result = maximum_consecutive_ones(&nums, k);
        assert_eq!(result, 0);
    }
}