// You have a long flowerbed in which some of the plots are planted, and some are not. 
// However, flowers cannot be planted in adjacent plots.

// Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, 
// and an integer n, return true if n new flowers can be planted in the flowerbed without 
// violating the no-adjacent-flowers rule and false otherwise.

pub fn can_place_flowers(flowerbed: &[i16], mut n: i32) -> bool {
    let mut f = flowerbed.to_vec();
    let len = f.len();

    for i in 0..len {
        if n <= 0 { break; }

        if f[i] == 0 {
            let prev_empty = i == 0 || f[i - 1] == 0;
            let next_empty = i == len - 1 || f[i + 1] == 0;

            if prev_empty && next_empty {
                f[i] = 1;
                n -= 1;
            }
        }
    }

    n <= 0
}

#[cfg(test)]
mod tests {
    use super::can_place_flowers;

    #[test]
    fn test_true_can_place_flowers(){
        let flowerbed = vec![1,0,0,0,1];
        let n = 1;
        let result = can_place_flowers(&flowerbed, n);
        assert_eq!(result, true);
    }

    #[test]
    fn test_false_can_place_flowers(){
        let flowerbed = vec![1,0,0,0,1];
        let n = 2;
        let result = can_place_flowers(&flowerbed, n);
        assert_eq!(result, false);
    }
}