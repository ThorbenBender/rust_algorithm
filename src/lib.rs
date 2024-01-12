fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums = nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();

    nums.sort_unstable_by_key(|&(_, n)| n);

    for (k, (i, ni)) in nums.iter().enumerate() {
        match nums[k + 1..].binary_search_by_key(&(target - ni), |&(_, nj)| nj) {
            Ok(jj) => return vec![*i as i32, nums[jj + k + 1].0 as i32],
            Err(_) => {}
        }
    }

    unreachable!("Error: this place should not be reachable");
    return vec![0, 0];
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
