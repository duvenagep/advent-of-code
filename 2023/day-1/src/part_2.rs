pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    const NUMS: [(&'static str, &'static str); 9] = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let digits = _input.lines().fold(0, |mut ans, _line| {
        let mut num_line = _line.to_string();

        for (num_str, replacement) in NUMS {
            num_line = num_line.replace(
                num_str,
                (num_str.to_string() + replacement + num_str).as_str(),
            );
        }

        println!("{:?}", num_line);

        let nums = num_line
            .chars()
            .filter_map(|x| x.to_digit(10))
            .collect::<Vec<u32>>();
        ans += 10 * nums.first().unwrap() + nums.last().unwrap();

        ans
    });
    Ok(digits.to_string())
}

#[cfg(test)]
mod tests {
    use crate::part_2::process;

    #[test]
    fn it_works() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let result = process(input).unwrap();
        let answer = "281".to_string();
        println!("Result: {result} & Answer: {answer}");
        assert_eq!(result, answer);
    }
}
