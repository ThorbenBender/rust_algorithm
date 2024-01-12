fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums = nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();

    nums.sort();

    let (mut left, mut right) = (0, nums.len() - 1);

    while left < right {
        let sum = nums[left].1 + nums[right].1;

        if sum == target {
            return vec![nums[left].0 as i32, nums[right].0 as i32];
        } else if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

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
