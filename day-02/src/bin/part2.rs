fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(input);
    println!("The answer is: {output}");
}

fn process(input: &str) -> i32 {
    input
        .lines()
        .map(parse_line_to_nums)
        .map(validate_numbers)
        .sum()
}

fn parse_line_to_nums(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn validate_numbers(mut nums: Vec<i32>) -> i32 {
    let mut first_fault = false;
    let mut i = 0;
    let is_ascending = nums[0] < nums[1];

    loop {
        if i >= nums.len() - 1 {
            break;
        }
        if compare_by_index(nums.clone(), i, i + 1, is_ascending) {
            if first_fault {
                return 0;
            }
            first_fault = true;
            nums.remove(i + 1);
            if nums.len() > i + 1 && compare_by_index(nums.clone(), i, i + 1, is_ascending) {
                return 0;
            }
            continue;
        }
        i += 1;
    }
    1
}

fn compare_by_index(collection: Vec<i32>, a: usize, b: usize, is_ascending: bool) -> bool {
    (collection[a] - collection[b]).abs() > 3
        || collection[a] == collection[b]
        || is_ascending && collection[a] > collection[b]
        || !is_ascending && collection[a] < collection[b]
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
        assert_eq!(result, parse_line_to_nums(&"1 2 3 4 5"));
    }
    #[test]
    fn it_validates_numbers() {
        assert_eq!(0, validate_numbers(vec![1, 1, 1, 1, 1]));
        assert_eq!(1, validate_numbers(vec![1, 2, 3, 4, 5]));
        assert_eq!(1, validate_numbers(vec![7, 6, 4, 2, 1]));
        assert_eq!(1, validate_numbers(vec![8, 6, 4, 4, 1]));
        assert_eq!(1, validate_numbers(vec![1, 3, 6, 7, 9]));
        assert_eq!(1, validate_numbers(vec![1, 3, 2, 4, 5]));
        assert_eq!(0, validate_numbers(vec![1, 2, 7, 8, 9]));
        assert_eq!(0, validate_numbers(vec![9, 7, 6, 2, 1]));
        // 449 is too low
        // 626
    }
}
