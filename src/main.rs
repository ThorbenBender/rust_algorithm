use rand::{self, Rng};

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());

    let (mut l, mut r) = (0, 0);

    while left.len() > l && right.len() > r {
        if left[l] > right[r] {
            result.push(right[r]);
            r += 1;
        } else {
            result.push(left[l]);
            l += 1;
        }
    }

    result.extend_from_slice(&left[l..]);
    result.extend_from_slice(&right[r..]);

    return result;
}

fn sort(numbers: &mut [i32]) {
    if numbers.len() > 1 {
        let middle = numbers.len() / 2;
        sort(&mut numbers[0..middle]);
        sort(&mut numbers[middle..]);

        let mut merged = merge(&numbers[..middle], &numbers[middle..]);
        numbers.swap_with_slice(&mut merged);
    }
}

fn main() {
    let mut numbers: Vec<i32> = (0..=100)
        .map(|_| rand::thread_rng().gen_range(0..=10000))
        .collect();
    println!("{:#?}", numbers);
    sort(&mut numbers);
    println!("{:?}", numbers);
}
