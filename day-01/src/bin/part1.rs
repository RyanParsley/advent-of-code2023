fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(input);
    println!("The answer is: {output}");
}

fn process(input: &str) -> i32 {
    let numbers = input
        .lines()
        .map(parse_numbers_from_string)
        .collect::<Vec<_>>();

    let mut left = numbers
        .clone()
        .into_iter()
        .map(|line| line.unwrap().0)
        .collect::<Vec<_>>();

    let mut right = numbers
        .clone()
        .into_iter()
        .map(|line| line.unwrap().1)
        .collect::<Vec<_>>();

    left.sort();
    right.sort();

    elementwise_subtraction(right, left).into_iter().sum()
}

fn parse_numbers_from_string(input: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return None; // Not exactly two numbers separated by whitespace
    }

    let num1 = parts[0].parse::<i32>().ok()?;
    let num2 = parts[1].parse::<i32>().ok()?;

    Some((num1, num2))
}

fn elementwise_subtraction(vec_a: Vec<i32>, vec_b: Vec<i32>) -> Vec<i32> {
    vec_a
        .into_iter()
        .zip(vec_b)
        .map(|(a, b)| (a - b).abs())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 11;
        assert_eq!(result, process(input));
    }
}
