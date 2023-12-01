pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let digits: Vec<_> = _input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect();

    let result: u32 = digits
        .iter()
        .map(|d| 10 * d.first().unwrap() + d.last().unwrap())
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use crate::part_1::process;

    #[test]
    fn it_works() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let result = process(input).unwrap();
        let answer = "142".to_string();
        println!("Result: {result} & Answer: {answer}");

        assert_eq!(result, answer);
    }
}
