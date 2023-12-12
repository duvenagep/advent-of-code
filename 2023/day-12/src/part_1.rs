use itertools::Itertools;
use std::str::FromStr;
use std::time::Instant;
use std::usize;

fn is_valid(springs: &str, instrs: &[usize]) -> bool {
    let springs = springs
        .split('.')
        .filter(|a| !a.is_empty())
        .collect::<Vec<_>>();
    if springs.len() != instrs.len() {
        return false;
    }

    springs
        .iter()
        .zip(instrs.iter())
        .all(|(spring, instr)| spring.len() == *instr)
}

fn build_grid(_input: &str) -> Vec<(String, Vec<usize>)> {
    let lines: Vec<_> = _input
        .lines()
        .map(|line| line.trim().split(' ').collect::<Vec<_>>())
        .map(|line| {
            (
                line[0].to_string(),
                line[1]
                    .split(',')
                    .map(|a| a.parse::<usize>().unwrap())
                    // .map(|a| a.unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    lines
}

fn get_combinations(springs: &str, instrs: &[usize]) -> usize {
    if springs.find('?').is_some() {
        let with_spring = springs.replacen('?', "#", 1);
        let without_spring = springs.replacen('?', ".", 1);
        // println!("{:?} \n {:?}", with_spring, without_spring);
        get_combinations(&with_spring, instrs) + get_combinations(&without_spring, instrs)
    } else {
        match is_valid(springs, instrs) {
            true => {
                println!("{:?}", springs);
                1
            }
            false => 0,
        }
    }
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let now = Instant::now();
    let lines = build_grid(_input);
    let result = lines
        .iter()
        .map(|(a, b)| get_combinations(a, b))
        .sum::<usize>()
        .to_string();

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");
    Ok(result.to_string())
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
