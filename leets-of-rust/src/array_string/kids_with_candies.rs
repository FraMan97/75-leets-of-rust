// There are n kids with candies. You are given an integer array candies, where each candies[i] represents 
// the number of candies the ith kid has, and an integer extraCandies, denoting the number of extra candies that you have.

// Return a boolean array result of length n, where result[i] is true if, after giving the ith kid all the extraCandies, they will have the greatest number of candies among all the kids, or false otherwise.

// Note that multiple kids can have the greatest number of candies.

pub fn kids_with_candies(candies: &[i64], extra_candies: i64) -> Vec<bool> {
    let max = match candies.iter().max() {
        Some(&m) => m,
        None => return vec![],
    };

    candies
        .iter()
        .map(|&kid| kid + extra_candies >= max)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::kids_with_candies;

    #[test]
    fn test_kids_with_candies(){
        let candies = vec![2,3,5,1,3];
        let extra_candies = 3;
        let result = kids_with_candies(&candies, extra_candies);
        assert_eq!(result, vec![true, true, true, false, true]);
    }

    #[test]
    fn test_empty_kids_with_candies(){
        let candies = vec![];
        let extra_candies = 3;
        let result = kids_with_candies(&candies, extra_candies);
        assert_eq!(result, vec![]);
    }
}