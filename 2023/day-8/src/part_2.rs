use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
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

    let mut node: Vec<String> = network
        .map
        .iter()
        .filter(|n| n.0.ends_with('A'))
        .map(|n| n.0.clone())
        .collect();

    let mut outputs = Vec::new();
    for el in node.iter_mut() {
        let mut idx = 0;
        let mut output = 0;

        while !el.ends_with('Z') {
            let direction = network.steps[idx];
            match direction {
                Step::Left => *el = network.map[el].0.clone(),
                Step::Right => *el = network.map[el].1.clone(),
                _ => panic!("Should not be possible"),
            };
            idx = (idx + 1) % network.steps.len();
            output += 1;
        }
        outputs.push(output);
    }

    let mut result: i64 = 1;
    for i in outputs {
        result = lcm(i, result);
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        let result = process(input).unwrap();
        let answer = "6".to_string();
        assert_eq!(result, answer);
    }
}
