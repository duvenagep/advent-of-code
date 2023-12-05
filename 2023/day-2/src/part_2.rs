pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let num_games =
        _input
            .lines()
            .map(|line| line.trim())
            .enumerate()
            .fold(0, |mut acc, (_i, _line)| {
                let mut _num_line = _line.to_string();

                let _var: String = _num_line
                    .split(": ")
                    .skip(1)
                    .map(|line| line.replace("; ", ", "))
                    .collect();

                let _splits: Vec<(u32, &str)> = _var
                    .split(", ")
                    .map(|el| {
                        let tokens: Vec<&str> = el.split(" ").collect();
                        (tokens[0].parse::<u32>().unwrap(), tokens[1])
                    })
                    .collect();

                let mut red: Vec<u32> = Vec::new();
                let mut green: Vec<u32> = Vec::new();
                let mut blue: Vec<u32> = Vec::new();

                for (num, colour) in _splits.clone() {
                    match colour {
                        "red" => red.push(num),
                        "green" => green.push(num),
                        "blue" => blue.push(num),
                        _ => continue,
                    }
                }

                let result = red.iter().max().unwrap()
                    * green.iter().max().unwrap()
                    * blue.iter().max().unwrap();
                acc += result;
                acc
            });
    Ok(num_games.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = process(input).unwrap();
        let answer = "2286".to_string();
        assert_eq!(result, answer);
    }
}
