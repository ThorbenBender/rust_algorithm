fn roman_to_int(s: String) -> i32 {
    let mut previos_worth: i32 = get_int(s.chars().next().unwrap());
    s.chars().fold(0, |number, c| {
        let current_worth = get_int(c);
        let calculated_worth = if current_worth > previos_worth {
            current_worth - previos_worth * 2
        } else {
            current_worth
        };
        previos_worth = current_worth;
        number + calculated_worth
    })
}

fn get_int(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::roman_to_int;
    #[test]
    pub fn test_two_sum() {
        let input = String::from("MCMXCIV");
        let expected = 1994;
        let result = roman_to_int(input);
        assert_eq!(result, expected);
    }
}
