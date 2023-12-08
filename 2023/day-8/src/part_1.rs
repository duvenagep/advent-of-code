use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Network {
    steps: Vec<Step>,
    map: HashMap<String, (String, String)>,
}

#[derive(Debug, Clone, Copy)]
pub enum Step {
    Left,
    Right,
}

pub fn parse_steps(line: &str) -> Vec<Step> {
    line.chars()
        .map(|c| match c {
            'L' => Step::Left,
            'R' => Step::Right,
            _ => panic!("Can't Happen"),
        })
        .collect::<Vec<_>>()
}

pub fn parse_network(lines: &[&str]) -> HashMap<String, (String, String)> {
    lines
        .iter()
        .map(|line| {
            let (key, value) = line.split_once(" = ").expect("Should split fine");
            let value = value.replace(['(', ')'], "");
            let (left_node, right_node) = value.split_once(", ").expect("Should Split Fine");
            (
                key.to_string(),
                (left_node.to_string(), right_node.to_string()),
            )
        })
        .collect::<HashMap<String, (String, String)>>()
}

pub fn parse(_input: &str) -> Network {
    let lines: Vec<_> = _input.lines().map(|line| line.trim()).collect();
    let instructions = lines[0];
    let nodes = &lines[2..];
    let steps = parse_steps(instructions);
    let map = parse_network(nodes);

    Network { steps, map }
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let network = parse(_input);
    println!("{:?}", network);

    let mut node = "AAA".to_string();
    let mut idx = 0;
    let mut output = 0;

    while node != "ZZZ" {
        let direction = network.steps[idx];
        match direction {
            Step::Left => node = network.map[&node].0.clone(),
            Step::Right => node = network.map[&node].1.clone(),
            _ => panic!("Should not be possible"),
        };
        idx = (idx + 1) % network.steps.len();
        output += 1;
    }

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

        let result = process(input).unwrap();
        let answer = "2".to_string();
        assert_eq!(result, answer);
    }
}
