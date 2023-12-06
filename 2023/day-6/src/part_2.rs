pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (_time, _distance) = _input.trim().split_once("\n").unwrap();

    let time = _time
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .filter(|c| !c.contains(" "))
        .fold("".to_string(), |mut acc, c| acc.to_string() + c)
        .parse::<i64>()
        .unwrap();
    let distance = _distance
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .filter(|c| !c.contains(" "))
        .fold("".to_string(), |mut acc, c| acc.to_string() + c)
        .parse::<i64>()
        .unwrap();

    println!("{:?} -- {:?}", time, distance);

    let mut race: Vec<i64> = Vec::new();
    for x in 0..=time {
        let d = x * (time - x);
        if d > distance {
            race.push(x);
        }
    }
    let (min, max) = (race.iter().min().unwrap(), race.iter().max().unwrap());

    let result = max - min + 1;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        let result = process(input).unwrap();
        let answer = "71503".to_string();
        assert_eq!(result, answer);
    }
}
