pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let _games = _input.lines().fold(0, |mut acc, lines| {
        let scratch: Vec<Vec<u32>> = lines
            .split(": ")
            .skip(1)
            .flat_map(|line| {
                line.split("|").map(|r| {
                    r.split_whitespace()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect()
                })
            })
            .collect();

        let lotto = scratch.get(0).unwrap();
        let game = scratch.get(1).unwrap();

        let count: u32 = lotto
            .iter()
            .filter(|&x| game.contains(x))
            .count()
            .try_into()
            .unwrap();

        println!("{:?}", count);

        if count > 2 {
            acc += u32::pow(2, count - 1)
        } else {
            acc += count
        }

        println!("{:?}", acc);

        acc
    });

    Ok(_games.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = process(input).unwrap();
        let answer = "13".to_string();
        assert_eq!(result, answer);
    }
}
