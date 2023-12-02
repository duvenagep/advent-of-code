const LIMIT: [(&'static u32, &'static str); 3] = [(&12, "red"), (&13, "green"), (&14, "blue")];

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let num_games =
        _input
            .lines()
            .map(|line| line.trim())
            .enumerate()
            .fold(0, |mut acc, (mut i, _line)| {
                i = i + 1;
                let mut _line = _line.to_string();

                let _var: String = _line
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

                println!("{:?}", _splits.clone());

                let mut result = Vec::new();

                for (num, cat) in _splits.clone() {
                    for (lim, cube) in LIMIT {
                        if cat == cube && &num <= lim {
                            result.push(1);
                        }
                    }
                }

                if _splits.clone().len() == result.len() {
                    acc += i
                }

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
        let answer = "8".to_string();
        assert_eq!(result, answer);
    }
}
