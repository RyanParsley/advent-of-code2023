fn main() {
    process()
}

fn process() {
    let input = include_str!("../../input2.txt");
    let sum: i32 = input
        .lines()
        .map(|line| replace_words(line))
        .map(|line| decode_string(&line))
        .sum();

    println!("The answer is: {}", sum);
}

fn decode_string(string: &str) -> i32 {
    let first = string.chars().find(|&c| c.is_digit(10)).unwrap();
    let last = string.chars().rev().find(|&c| c.is_digit(10)).unwrap();

    format!("{}{}", first, last).parse::<i32>().unwrap()
}

fn replace_words(string: &str) -> String {
    let parsed: Vec<&str> = string
        .chars()
        .enumerate()
        .map(|(i, _)| string.get(i..).expect("well, that's embarassing"))
        .map(|token| word_parser(token))
        .filter(|word| word.len() > 0)
        .collect();

    parsed.join("")
}
fn word_parser(word: &str) -> &str {
    if word.starts_with("one") {
        return "1";
    }
    if word.starts_with("two") {
        return "2";
    }
    if word.starts_with("three") {
        return "3";
    }
    if word.starts_with("four") {
        return "4";
    }
    if word.starts_with("five") {
        return "5";
    }
    if word.starts_with("six") {
        return "6";
    }
    if word.starts_with("seven") {
        return "7";
    }
    if word.starts_with("eight") {
        return "8";
    }
    if word.starts_with("nine") {
        return "9";
    }
    if word.chars().nth(0).is_some_and(|char| char.is_digit(10)) {
        return &word[..1];
    }
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_replacement_works() {
        let input = "onefootwo";
        let result = "12".to_string();
        assert_eq!(result, replace_words(input));

        let input = "eightwothree";
        let result = "823".to_string();
        assert_eq!(result, replace_words(input));
    }
}
