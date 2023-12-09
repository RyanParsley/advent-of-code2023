fn main() {
    let input = include_str!("../../input2.txt");

    let sum = process(input);
    println!("The answer is: {sum}");
}

fn process(input: &str) -> i32 {
    input
        .lines()
        .map(|line| parse_cubes(line))
        .filter(|game| is_game_valid(game))
        .map(|line| line.id)
        .sum()
}

#[derive(Debug)]
struct Cubes {
    blue: i32,
    red: i32,
    green: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Cubes>,
}

fn parse_cubes(line: &str) -> Game {
    let game = line.split(":").collect::<Vec<&str>>();

    let id = game[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();

    let game = game[1].trim().split(";").collect::<Vec<&str>>();

    let rounds: Vec<Cubes> = game
        .iter()
        .map(|round| {
            round.split(',').map(|cube| cube.trim()).fold(
                Cubes {
                    red: 0,
                    blue: 0,
                    green: 0,
                },
                |mut acc, cube| {
                    let set = cube.split(" ").collect::<Vec<&str>>();
                    match set[1] {
                        "green" => acc.green += set[0].parse::<i32>().unwrap(),
                        "blue" => acc.blue += set[0].parse::<i32>().unwrap(),
                        "red" => acc.red += set[0].parse::<i32>().unwrap(),
                        _ => println!("I am Error"),
                    }
                    acc
                },
            )
        })
        .collect();

    Game { id, rounds }
}

fn is_game_valid(game: &Game) -> bool {
    game.rounds.iter().fold(true, |acc, round| {
        if acc == false {
            return false;
        }
        round.blue <= 14 && round.red <= 12 && round.green <= 13
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 2286;
        assert_eq!(result, process(input));
    }
}
