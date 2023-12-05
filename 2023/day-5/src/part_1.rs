pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    todo!("day_5 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "INPUT GOES HERE";

        let result = process(input).unwrap();
        let answer = "ANSWER GOES HERE".to_string();
        assert_eq!(result, answer);
    }
}
