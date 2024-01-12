fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut m: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        match m.get(&(target - *num)) {
            Some(&ix) => return vec![ix, i as i32],
            None => m.insert(*num, i as i32),
        };
    }
    return vec![];
}

#[cfg(test)]
mod tests {
    use crate::two_sum;
    #[test]
    pub fn test_two_sum() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(numbers, target);
        assert_eq!(result, vec![0, 1]);
    }
}
