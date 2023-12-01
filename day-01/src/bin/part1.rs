fn main() {
    process()
}

fn process() {
    let input = include_str!("../../input1.txt");
    let sum = input.lines().fold(0, |acc, line| acc + decode_string(line));
    println!("Hello, world! {}", sum);
}

fn decode_string(string: &str) -> i32 {
    let first = string.chars().find(|&c| c.is_digit(10)).ok_or("0").unwrap();
    let last = string
        .chars()
        .rev()
        .find(|&c| c.is_digit(10))
        .ok_or("0")
        .unwrap();
    let result = format!("{}{}", first, last);
    result.parse::<i32>().unwrap()
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
