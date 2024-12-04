fn main() {
    let input = include_str!("../../input1.txt");
    //let input = include_str!("mock-1.txt");
    let output = process(input);
    println!("The answer is: {output}");
}

fn process(input: &str) -> i32 {
    parse_string_numbers(input)
        .into_iter()
        .map(|(a, b)| a * b)
        .sum()
}

fn parse_string_numbers(input: &str) -> Vec<(i32, i32)> {
    let result = input
        .split("mul(")
        .map(|chunk| chunk.split(")").collect::<Vec<_>>()[0])
        .map(|chunk| chunk.split(",").collect::<Vec<_>>())
        .filter(|chunk| chunk.len() == 2)
        .map(|chunk| {
            (
                chunk[0].parse::<i32>().unwrap_or(0),
                chunk[1].parse::<i32>().unwrap_or(0),
            )
        })
        .collect::<Vec<_>>();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 161;
        assert_eq!(result, process(input));
        // too high 160683556
    }

    #[test]
    fn it_parses_a_bad_string_to_nothing() {
        let input = "what()mul(758,157(][&+how()";
        let result = vec![(758, 0)];
        assert_eq!(result, parse_string_numbers(input));
        // too high 160683556
    }
}
