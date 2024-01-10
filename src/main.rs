use rand::{self, Rng};

fn select_sort(numbers: &mut Vec<i32>) {
    for i in 0..numbers.len() {
        let mut smallest = i;
        for j in i + 1..numbers.len() {
            println!("Values i: {} j: {}", numbers[i], numbers[j]);
            if numbers[j] < numbers[smallest] {
                smallest = j;
            }
        }
        if smallest == i {
            numbers.swap(i, smallest);
        }
    }
}

fn bubble_sort(numbers: &mut Vec<i32>) {
    for _ in 0..numbers.len() {
        let mut swapped = false;
        for j in 0..numbers.len() - 1 {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j + 1, j);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

fn merge(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    while left.len() > 0 && right.len() > 0 {
        if left[0] > right[0] {
            result.push(right.remove(0));
        } else {
            result.push(left.remove(0));
        }
    }
    if right.len() > 0 {
        result.append(&mut right);
    }
    if left.len() > 0 {
        result.append(&mut left);
    }
    return result;
}

fn sort(numbers: Vec<i32>) -> Vec<i32> {
    if numbers.len() > 1 {
        let middle = numbers.len() / 2;
        let left = sort(numbers[0..middle].to_vec());
        let right = sort(numbers[middle..numbers.len()].to_vec());

        return merge(left, right);
    } else {
        return numbers;
    }
}

fn main() {
    let numbers: Vec<i32> = (0..=1000000)
        .map(|_| rand::thread_rng().gen_range(0..=10000))
        .collect();
    println!("{:#?}", numbers);
    let sorted = sort(numbers);
    println!("{:?}", sorted);
}
