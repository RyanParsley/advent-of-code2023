
fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(input);
    println!("The answer is: {output}");
}

fn process(input: &str) -> i32 {
    input.lines().map(parse_line_to_nums).map(validate_numbers).sum()
}

fn parse_line_to_nums(line: &str) -> Vec<i32> {
    line.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn validate_numbers(nums: Vec<i32>) -> i32 {
    let isAscending = nums[0] < nums[1]; 
    for i in 0..nums.len()-1 {
       if (nums[i] - nums[i+1]).abs() > 3 { return 0; }
       if nums[i] == nums[i+1] { return 0; }
            if isAscending && nums[i]>nums[i+1] { return 0;}
            if !isAscending && nums[i]<nums[i+1] { return 0;}
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 2;
        assert_eq!(result, process(input));
    }

    #[test]
    fn it_parses_lines_to_numbers () {
        let result = vec![1,2,3,4,5];
        assert_eq!(result, parse_line_to_nums(&"1 2 3 4 5"));
    }
    #[test]
    fn it_validates_numbers () {
        assert_eq!(0, validate_numbers(vec![1,1,1,1,1]));
        assert_eq!(1, validate_numbers(vec![1,2,3,4,5]));
        assert_eq!(1, validate_numbers(vec![7,6,4,2,1]));
        assert_eq!(0, validate_numbers(vec![1,2,7,8,9]));
        assert_eq!(0, validate_numbers(vec![9,7,6,2,1]));
        assert_eq!(0, validate_numbers(vec![1,3,2,4,5]));
        assert_eq!(0, validate_numbers(vec![8,6,4,4,1]));
        assert_eq!(1, validate_numbers(vec![1,3,6,7,9]));
    }
}
