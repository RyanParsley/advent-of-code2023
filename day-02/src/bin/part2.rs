fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(input);
    println!("The answer is: {output}");
}

fn process(input: &str) -> i32 {
    input
        .lines()
        .map(parse_line_to_nums)
        .map(is_this_fine)
        .filter(|x| *x)
        .count() as i32
}

fn is_this_fine(nums: Vec<i32>) -> bool {
    if validate_numbers(nums.clone()) {
        return true;
    }
    for i in 0..nums.len() {
        let mut nums_copy = nums.clone();
        nums_copy.remove(i);
        if validate_numbers(nums_copy) {
            return true;
        }
    }
    false
}

fn parse_line_to_nums(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn validate_numbers(nums: Vec<i32>) -> bool {
    let is_ascending = nums.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = nums.windows(2).all(|w| w[0] >= w[1]);
    if !is_ascending && !is_descending {
        return false;
    }

    let is_valid = nums
        .windows(2)
        .map(|window| (window[0] - window[1]).abs())
        .all(|diff| (1..=3).contains(&diff));
    is_valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 4;
        assert_eq!(result, process(input));
    }

    #[test]
    fn it_parses_lines_to_numbers() {
        let result = vec![1, 2, 3, 4, 5];
        assert_eq!(result, parse_line_to_nums("1 2 3 4 5"));
    }
    #[test]
    fn it_validates_numbers() {
        assert!(validate_numbers(vec![1, 1, 1, 1, 1]));
        assert!(!validate_numbers(vec![1, 2, 3, 4, 5]));
        assert!(!validate_numbers(vec![7, 6, 4, 2, 1]));
        assert!(!validate_numbers(vec![8, 6, 4, 4, 1]));
        assert!(!validate_numbers(vec![1, 3, 6, 7, 9]));
        assert!(!validate_numbers(vec![1, 3, 2, 4, 5]));
        assert!(validate_numbers(vec![1, 2, 7, 8, 9]));
        assert!(validate_numbers(vec![9, 7, 6, 2, 1]));
        // 465
    }
}
