fn main() {
    let input = include_str!("../../input2.txt");

    println!("The answer is:");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "foo";
        let result = "bar";
        assert_eq!(result, input);
    }
}
