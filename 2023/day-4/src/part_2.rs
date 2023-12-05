use std::collections::HashMap;

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut car_map: HashMap<usize, usize> = HashMap::new();
    let _games = _input
        .lines()
        .enumerate()
        .fold(0, |mut acc, (mut idx, lines)| {
            idx = idx + 1;
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

            let count = lotto.iter().filter(|x| game.contains(x)).count();
            car_map.insert(idx, count);
            acc
        });
    println!("{:?}", car_map);

    let part_2: usize = car_map
        .keys()
        .map(|card_number| find_total(*card_number, &car_map))
        .sum::<usize>();

    Ok(part_2.to_string())
}

pub fn find_total(card_number: usize, scoring: &HashMap<usize, usize>) -> usize {
    let matches = *scoring.get(&card_number).unwrap();

    ((card_number + 1)..(card_number + matches + 1))
        .map(|card_number| find_total(card_number, scoring))
        .sum::<usize>()
        + 1
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
        let answer = "30".to_string();
        assert_eq!(result, answer);
    }
}
