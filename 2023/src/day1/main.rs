use std::fs;

fn main() {
    println!("AOC day 1");

    // let input = "1abc2
    // pqr3stu8vwx
    // a1b2c3d4e5f
    // treb7uchet";

    let input1 = fs::read_to_string("./src/day1/input1.txt").expect("File should exist");
    let digits: Vec<_> = input1
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

    println!("Part 1 = {:?}", result);

    // let input2 = "two1nine
    // eightwothree
    // abcone2threexyz
    // xtwone3four
    // 4nineeightseven2
    // zoneight234
    // 7pqrstsixteen";

    let input2 = fs::read_to_string("./src/day1/input2.txt").expect("File should exist");

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

    let digits = input2.lines().fold(0, |mut ans, _line| {
        let mut num_line = _line.to_string();

        for (num_str, replacement) in NUMS {
            num_line = num_line.replace(
                num_str,
                (num_str.to_string() + replacement + num_str).as_str(),
            );
        }
        let nums = num_line
            .chars()
            .filter_map(|x| x.to_digit(10))
            .collect::<Vec<u32>>();
        ans += 10 * nums.first().unwrap() + nums.last().unwrap();

        ans
    });

    println!("Part 2 = {:?}", digits);
}
