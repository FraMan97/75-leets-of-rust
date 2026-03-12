// Given an array of integers nums, calculate the pivot index of this array.

// The pivot index is the index where the sum of all the numbers strictly to the left of the index is 
// equal to the sum of all the numbers strictly to the index's right.

// If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left. 
// This also applies to the right edge of the array.

// Return the leftmost pivot index. If no such index exists, return -1.
pub fn find_pivot_index(nums: &[i64]) -> i64 {
    let total_sum: i64 = nums.iter().sum();
    let mut left_sum = 0;
    for (index, &el) in nums.iter().enumerate() {
        if left_sum == total_sum - el -left_sum {
            return index as i64;
        }
        left_sum += el;

    }
    -1
}

#[cfg(test)]
mod tests {
    use super::find_pivot_index;

    #[test]
    fn test_find_pivot_index(){
        let nums = [1,7,3,6,5,6];
        let result = find_pivot_index(&nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_empty_find_pivot_index(){
        let nums = [];
        let result = find_pivot_index(&nums);
        assert_eq!(result, -1);
    }
}