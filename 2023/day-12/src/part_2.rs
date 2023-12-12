use itertools::Itertools;
use std::time::Instant;

fn is_valid(vec: &Vec<&String>) -> bool {
    let indexes_of_hash: Vec<usize> = vec
        .iter()
        .enumerate()
        .filter(|(_, &elem)| elem.contains("#"))
        .map(|(index, _)| index)
        .collect();

    let mut v = Vec::new();

    for i in 1..indexes_of_hash.len() {
        if indexes_of_hash[i] - indexes_of_hash[i - 1] <= 1 {
            v.push(false);
        } else {
            v.push(true);
        }
    }

    v.iter().contains(&false)
}

fn build_grid(_input: &str) -> Vec<(&str, Vec<u8>)> {
    let grid: Vec<(&str, Vec<u8>)> = _input
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let (chars, numbers) = line.split_once(" ").unwrap();
            let chars: &str = chars.trim();
            let numbers: Vec<u8> = numbers
                .trim()
                .split(",")
                .map(|num| num.parse::<u8>().unwrap())
                .collect();
            (chars, numbers)
        })
        .collect();
    grid
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    todo!();
    // let now = Instant::now();
    // let grid = build_grid(_input);

    // let (char_one, num_one) = grid[0].clone();
    // println!("{:?} - {:?}", char_one, num_one);

    // let comb: Vec<String> = num_one
    //     .iter()
    //     .map(|el| "#".repeat(*el as usize))
    //     .chain(
    //         (0..(char_one.chars().count() - num_one.iter().sum::<u8>() as usize))
    //             .map(|_| ".".to_string()),
    //     )
    //     .collect();

    // // println!("{:?}", comb);

    // for perm in comb
    //     .iter()
    //     .permutations(comb.len())
    //     .unique()
    //     .filter(|perm| !is_valid(perm))
    // {
    //     let counts: Vec<u8> = perm
    //         .iter()
    //         .filter(|c| c.contains("#"))
    //         .map(|c| c.chars().count() as u8)
    //         .collect();

    //     if counts == num_one {
    //         println!("{:?}", perm);
    //     }
    // }

    // let elapsed = now.elapsed();
    // println!("Elapsed: {elapsed:.4?}");
    // Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

        let result = process(input).unwrap();
        let answer = "21".to_string();
        assert_eq!(result, answer);
    }
}
