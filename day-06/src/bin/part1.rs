fn main() {
    let input = include_str!("../../input1.txt");
    let answer = process(input);
    println!("The answer is: {answer}");
}

struct Race {
    time: i32,
    distance: i32,
}

fn process(input: &str) -> i32 {
    let times: Vec<i32> = input.lines().collect::<Vec<&str>>()[0]
        .split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .iter()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let distances: Vec<i32> = input.lines().collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .iter()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let races = times
        .iter()
        .enumerate()
        .map(|(index, time)| Race {
            time: *time,
            distance: distances[index],
        })
        .collect::<Vec<Race>>();

    races
        .iter()
        .map(|race| {
            let mut score = 0;
            for i in 1..race.time {
                if i * (race.time - i) > race.distance {
                    score += 1
                }
            }
            score
        })
        .fold(1, |acc, solutions| acc * solutions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 288;
        assert_eq!(result, process(input));

        let input = include_str!("../../input1.txt");
        let result = 2612736;
        assert_eq!(result, process(input));
    }
}
