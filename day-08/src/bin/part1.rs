fn main() {
    let input = include_str!("../../input1.txt");
    let answer = process(input);
    println!("The answer is: {answer}");
}

struct Node<'a> {
    key: &'a str,
    left: &'a str,
    right: &'a str,
}

fn process(input: &str) -> i32 {
    let _instructions: Vec<char> = input.lines().collect::<Vec<&str>>()[0]
        .chars()
        .collect::<Vec<char>>();

    let nodes: Vec<Node> = input.lines().collect::<Vec<&str>>()[2..]
        .into_iter()
        .map(|line| {
            let key = line.split("=").collect::<Vec<&str>>()[0].trim();
            dbg!(key);
            let destinations = line.split("=").collect::<Vec<&str>>()[1]
                .trim()
                .split(",")
                .collect::<Vec<&str>>();
            dbg!(&destinations);

            let left = destinations[0];
            let right = destinations[1];

            Node {
                key,
                left: &left,
                right: &right,
            }
        })
        .map(|_| Node {
            key: "AAA",
            left: "BBB",
            right: "CCC",
        })
        .collect::<Vec<Node>>();

    nodes.iter().map(|_| 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 2;
        assert_eq!(result, process(input));

        let input = include_str!("mock-2.txt");
        let result = 6;
        assert_eq!(result, process(input));
    }
}
