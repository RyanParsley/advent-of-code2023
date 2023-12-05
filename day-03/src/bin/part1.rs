#[derive(Debug, PartialEq)]
struct Symbol {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct Part {
    row: i32,
    col: i32,
    value: i32,
}

fn main() {
    let input = include_str!("../../input1.txt");
    let sum = process(input);
    println!("The answer is: {sum}");
}

fn process(input: &str) -> i32 {
    let special_chars = find_special_chars(input);
    find_numbers(input)
        .iter()
        .filter(|number| {
            // Todo: filter if not touching a symbol
            // special_chars.contains a value of previous row above before or after, same row before or after or next row below before or after
            // assume (x,y) or (col, row)
            let mut tangents = vec![(-1, -1), (-1, 0), (-1, 1), (1, -1), (1, 0), (1, 1)];
            for i in 0..=number.value.to_string().len() as i32 {
                tangents.push((i, -1));
                tangents.push((i, 1));
                tangents.push((i, 0));
            }
            dbg!(number);
            dbg!(&tangents);

            tangents
                .iter()
                .map(|tangent| Symbol {
                    row: number.row + tangent.1,
                    col: number.col + tangent.0,
                })
                .fold(false, |acc, tangent| {
                    if acc {
                        return acc;
                    }
                    special_chars.contains(&tangent)
                })
        })
        .map(|num| num.value)
        .sum()
}

fn find_special_chars(input: &str) -> Vec<Symbol> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.match_indices(|char: char| !char.is_digit(10) && char != '.')
                .map(|location| Symbol {
                    row: row as i32,
                    col: location.0 as i32,
                })
                .collect::<Vec<Symbol>>()
        })
        .collect::<Vec<Symbol>>()
}

fn find_numbers(input: &str) -> Vec<Part> {
    let parts = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.match_indices(|char: char| char.is_digit(10))
                .map(|(col, value)| Part {
                    row: row as i32,
                    col: col as i32,
                    value: value.parse::<i32>().unwrap(),
                })
                .collect::<Vec<Part>>()
        })
        .fold(vec![], |mut acc: Vec<Part>, part| {
            match acc.last_mut() {
                Some(valid_part) => {
                    if valid_part.row == part.row
                        && valid_part.col + valid_part.value.to_string().len() as i32 == part.col
                    {
                        valid_part.value = valid_part.value * 10 + part.value;
                    } else {
                        acc.push(part);
                    }
                }
                None => acc.push(part),
            }
            acc
        });
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 4361;
        assert_eq!(result, process(input));
    }

    #[test]
    fn it_finds_special_chars() {
        let input = include_str!("mock-1.txt");
        assert_eq!(6, find_special_chars(input).len());
        assert_eq!(3, find_special_chars(input)[0].col);
        assert_eq!(1, find_special_chars(input)[0].row);
    }

    #[test]
    fn it_finds_digits() {
        let input = include_str!("mock-1.txt");
        let result = Part {
            row: 0,
            col: 0,
            value: 467,
        };
        assert_eq!(result.value, find_numbers(input)[0].value);
        assert_eq!(result.col, find_numbers(input)[0].col);

        let result = Part {
            row: 0,
            col: 5,
            value: 114,
        };
        assert_eq!(result.value, find_numbers(input)[1].value);
        assert_eq!(result.col, find_numbers(input)[1].col);

        let result = Part {
            row: 0,
            col: 2,
            value: 35,
        };
        assert_eq!(result.value, find_numbers(input)[2].value);
        assert_eq!(result.col, find_numbers(input)[2].col);
    }
}
