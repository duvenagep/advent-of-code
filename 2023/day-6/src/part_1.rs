use std::iter::zip;

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lines: Vec<&str> = _input.lines().collect();
    let times: Vec<i32> = lines
        .get(0)
        .unwrap()
        .trim()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let distance: Vec<i32> = lines
        .get(1)
        .unwrap()
        .trim()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    println!("{:?} {:?}", times, distance);

    let mut records: Vec<i32> = Vec::new();

    for (i, j) in zip(times, distance) {
        let mut beat = 0;
        for x in 0..=i {
            let d = x * (i - x);
            if d > j {
                beat += 1;
            }
        }
        records.push(beat);
    }
    let combinations: i32 = records.iter().fold(1, |acc, x| acc * x);

    Ok(combinations.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        let result = process(input).unwrap();
        let answer = "288".to_string();
        assert_eq!(result, answer);
    }
}
