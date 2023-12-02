fn main() {
    process()
}

fn process() {
    let input = include_str!("../../input1.txt");
    let sum: i32 = input.lines().map(|line| decode_string(line)).sum();

    println!("The answer is: {}", sum);
}

fn decode_string(string: &str) -> i32 {
    let first = string.chars().find(|&c| c.is_digit(10)).unwrap();
    let last = string.chars().rev().find(|&c| c.is_digit(10)).unwrap();

    format!("{first}{last}").parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1abc2".to_string();
        let result = 12;
        assert_eq!(result, decode_string(&input));
    }
}
