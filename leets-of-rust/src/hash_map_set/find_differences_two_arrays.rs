// Given two 0-indexed integer arrays nums1 and nums2, return a list answer of size 2 where:

//    answer[0] is a list of all distinct integers in nums1 which are not present in nums2.
//    answer[1] is a list of all distinct integers in nums2 which are not present in nums1.

// Note that the integers in the lists may be returned in any order.
use std::collections::HashSet;

pub fn find_differences_two_arrays(nums1: &[i64], nums2: &[i64]) -> Vec<Vec<i64>> {
    let set1: HashSet<i64> = nums1.iter().copied().collect();
    let set2: HashSet<i64> = nums2.iter().copied().collect();

    let mut diff1: Vec<i64> = Vec::new();
    for &el in &set1 {
        if !set2.contains(&el) {
            diff1.push(el);
        }
    }

    let mut diff2: Vec<i64> = Vec::new();
    for &el in &set2 {
        if !set1.contains(&el) {
            diff2.push(el);
        }
    }

    vec![diff1, diff2]
}

#[cfg(test)]
mod tests {
    use super::find_differences_two_arrays;

    #[test]
    fn test_find_differences_two_arrays(){
        let nums1 = [1,2,3];
        let nums2 = [2,4,6];
        let mut result = find_differences_two_arrays(&nums1, &nums2);
        result[0].sort_unstable();
        result[1].sort_unstable();
        assert_eq!(result, vec![vec![1, 3], vec![4,6]]);
    }

    #[test]
    fn test_empty_find_differences_two_arrays(){
        let nums1 = [];
        let nums2 = [];
        let result = find_differences_two_arrays(&nums1, &nums2);
        assert_eq!(result, vec![vec![], vec![]]);
    }
}