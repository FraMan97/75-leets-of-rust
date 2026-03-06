// You are given an integer array nums consisting of n elements, and an integer k.

// Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. 
// Any answer with a calculation error less than 10-5 will be accepted.
pub fn maximum_average_subarray(nums: &[i32], k: usize) -> f64 {
    let mut current_sum: i32 = nums.iter().take(k).sum();
    let mut max_sum = current_sum;

    for i in k..nums.len() {
        current_sum += nums[i] - nums[i - k];
        
        max_sum = i32::max(current_sum, max_sum);
    }

    max_sum as f64 / k as f64
}

#[cfg(test)]
mod tests {
    use super::maximum_average_subarray;

    #[test]
    fn test_maximum_average_subarray(){
        let nums = [1,12,-5,-6,50,3];
        let k: usize = 4;
        let result = maximum_average_subarray(&nums, k);
        assert_eq!(result, 12.75000);
    }

    #[test]
    fn test_empty_maximum_average_subarray(){
        let nums = [];
        let k: usize = 4;
        let result = maximum_average_subarray(&nums, k);
        assert_eq!(result, 0.0);
    }
}