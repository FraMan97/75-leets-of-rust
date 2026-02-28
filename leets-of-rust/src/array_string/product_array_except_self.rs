// Given an integer array nums, return an array answer such that answer[i] is equal to the 
// product of all the elements of nums except nums[i].

// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

// You must write an algorithm that runs in O(n) time and without using the division operation.

pub fn product_array_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    if n == 0 { return vec![]; }

    let mut answers = vec![1; n];

    let mut left_mult = 1;
    for i in 0..n {
        answers[i] = left_mult;
        left_mult *= nums[i];
    }

    let mut right_mult = 1;
    for i in (0..n).rev() { 
        answers[i] *= right_mult; 
        right_mult *= nums[i];
    }

    answers
}


#[cfg(test)]
mod tests {
    use super::product_array_except_self;

    #[test]
    fn test_product_array_except_self(){
        let nums = vec![1,2,3,4];
        let result = product_array_except_self(&nums);
        assert_eq!(result, vec![24,12,8,6]);
    }

    #[test]
    fn test_empty_product_array_except_self(){
        let nums = vec![];
        let result = product_array_except_self(&nums);
        assert_eq!(result, vec![]);
    }
}