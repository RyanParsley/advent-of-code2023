fn main() {
    let input = include_str!("../../input1.txt");
    let answer = process(input);
    println!("The answer is: {answer}");
}

fn process(input: &str) -> i32 {
    input.lines().map(|_| 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 42;
        assert_eq!(result, process(input));
    }
}
