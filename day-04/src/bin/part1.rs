fn main() {
    let input = include_str!("../../input1.txt");
    let sum = process(input);
    println!("The answer is: {sum}");
}

fn process(input: &str) -> i32 {
    input
        .lines()
        .map(|card| {
            let card: Vec<&str> = card.split(":").collect::<Vec<&str>>()[1]
                .split("|")
                .collect::<Vec<&str>>();

            let winners: Vec<&str> = card[0].split_whitespace().collect::<Vec<&str>>();
            let ours: Vec<_> = card[1].split_whitespace().collect::<Vec<_>>();

            let our_winners = ours
                .iter()
                .filter(|number| winners.contains(number))
                .collect::<Vec<_>>();

            if our_winners.len() == 0 {
                return 0;
            }

            let base: i32 = 2;
            base.pow((our_winners.len() as u32) - 1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 13;
        assert_eq!(result, process(input));

        let input = include_str!("../../input1.txt");
        let result = 18619;
        assert_eq!(result, process(input));
    }
}
