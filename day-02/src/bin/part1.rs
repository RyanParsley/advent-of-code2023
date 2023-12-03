fn main() {
    let input = include_str!("../../input2.txt");

    println!("The answer is:");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = "bar";
        assert_eq!(result, input);
    }
}
