fn main() {
    let input = include_str!("../../input1.txt");
    let answer = process(input);
    println!("The answer is: {answer}");
}

#[derive(Debug, Copy, Clone)]
struct Map {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

fn process(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();

    let seeds: Vec<i64> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let maps: Vec<Vec<Map>> =
        lines[2..]
            .iter()
            .filter(|line| line != &&"")
            .fold(vec![], |mut acc, line| {
                if line.contains("map:") {
                    acc.push(vec![]);
                } else {
                    let nums: Vec<_> = line
                        .split_whitespace()
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect();

                    match acc.last_mut() {
                        Some(last_vec) => last_vec.push(Map {
                            destination_range_start: nums[0],
                            source_range_start: nums[1],
                            range_length: nums[2],
                        }),
                        None => {}
                    }
                };
                acc
            });

    let answer: i64 = seeds
        .into_iter()
        .map(|seed| {
            maps.iter()
                .fold(seed, |mut acc: i64, map_group: &Vec<Map>| {
                    acc = map_source_to_destination(acc, map_group);
                    acc
                })
        })
        .map(|foo| foo)
        .min()
        .unwrap();

    answer
}

fn map_source_to_destination(source: i64, maps: &Vec<Map>) -> i64 {
    maps.iter()
        .fold((source, false), |acc, map| {
            if acc.1 {
                return acc;
            }
            if acc.0 >= map.source_range_start
                && acc.0 < (map.source_range_start + map.range_length)
            {
                return (
                    map.destination_range_start + acc.0 - map.source_range_start,
                    true,
                );
            }
            acc
        })
        .0
}

// Map source to destination
// destination_range_start source_range_start range_length

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 35;
        assert_eq!(result, process(input));

        let input = include_str!("../../input1.txt");
        let result = 1181555926;
        assert_eq!(result, process(input));
    }

    #[test]
    fn it_maps_source_to_destinations() {
        let input = 13;
        let result = 13;
        let maps = vec![
            Map {
                source_range_start: 98,
                destination_range_start: 50,
                range_length: 2,
            },
            Map {
                source_range_start: 50,
                destination_range_start: 52,
                range_length: 48,
            },
        ];

        assert_eq!(result, map_source_to_destination(input, &maps));

        let input = 55;
        let result = 57;

        assert_eq!(result, map_source_to_destination(input, &maps));

        let input = 14;
        let result = 14;

        assert_eq!(result, map_source_to_destination(input, &maps));

        let input = 79;
        let result = 81;

        assert_eq!(result, map_source_to_destination(input, &maps));

        let maps = vec![
            Map {
                destination_range_start: 0,
                source_range_start: 15,
                range_length: 37,
            },
            Map {
                destination_range_start: 37,
                source_range_start: 52,
                range_length: 2,
            },
            Map {
                destination_range_start: 39,
                source_range_start: 0,
                range_length: 15,
            },
        ];

        let input = 81;
        let result = 81;

        assert_eq!(result, map_source_to_destination(input, &maps));

        let input = 14;
        let result = 53;

        assert_eq!(result, map_source_to_destination(input, &maps));

        let input = 57;
        let result = 57;

        assert_eq!(result, map_source_to_destination(input, &maps));

        let input = 13;
        let result = 52;

        assert_eq!(result, map_source_to_destination(input, &maps));

        // I'm making some test data here

        let maps = vec![Map {
            destination_range_start: 10,
            source_range_start: 0,
            range_length: 37,
        }];

        let input = 9;
        let result = 19;

        assert_eq!(result, map_source_to_destination(input, &maps));

        let maps = vec![
            Map {
                destination_range_start: 0,
                source_range_start: 15,
                range_length: 37,
            },
            Map {
                destination_range_start: 37,
                source_range_start: 52,
                range_length: 2,
            },
            Map {
                destination_range_start: 39,
                source_range_start: 0,
                range_length: 15,
            },
        ];

        let input = 14;
        let result = 53;

        assert_eq!(result, map_source_to_destination(input, &maps));

        let maps = vec![
            Map {
                destination_range_start: 49,
                source_range_start: 53,
                range_length: 8,
            },
            Map {
                destination_range_start: 0,
                source_range_start: 11,
                range_length: 42,
            },
            Map {
                destination_range_start: 42,
                source_range_start: 0,
                range_length: 7,
            },
            Map {
                destination_range_start: 57,
                source_range_start: 7,
                range_length: 4,
            },
        ];

        let input = 53;
        let result = 49;

        assert_eq!(result, map_source_to_destination(input, &maps));
    }
}
