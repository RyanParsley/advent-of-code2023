
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

    let left = numbers
        .clone()
        .into_iter()
        .map(|line| line.unwrap().0)
        .collect::<Vec<_>>();

    let right = numbers
        .clone()
        .into_iter()
        .map(|line| line.unwrap().1)
        .collect::<Vec<_>>();

    left.into_iter().map(|value| value * count_occurances(value, right.clone())).sum()
}

fn count_occurances(num: i32, collection: Vec<i32>) -> i32 {
    collection.iter().filter(|&n| *n == num).count() as  i32
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 31;
        assert_eq!(result, process(input));
    }
}
