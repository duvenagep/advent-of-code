use std::collections::HashMap;

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (rules, updates) = _input.trim().split_once("\n\n").unwrap();

    let (before, after): (HashMap<u32, Vec<u32>>, HashMap<u32, Vec<u32>>) = rules.lines().fold(
        (HashMap::new(), HashMap::new()),
        |(mut before, mut after), line| {
            let (l, r) = line
                .trim()
                .split_once("|")
                .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
                .unwrap();
            before.entry(l).and_modify(|v| v.push(r)).or_insert(vec![r]);
            after.entry(r).and_modify(|v| v.push(l)).or_insert(vec![l]);
            (before, after)
        },
    );

    let result: u32 = updates
        .lines()
        .map(|line| {
            let mut correct = true;
            let numbers: Vec<u32> = line
                .trim()
                .split(",")
                .map(|d| d.parse::<u32>().unwrap())
                .collect();

            for (number, befores) in numbers.iter().zip(numbers.iter().skip(1)) {
                if let Some(before) = before.get(&number) {
                    if !before.contains(&befores) {
                        correct = false
                    }
                }

                if let Some(after) = after.get(&befores) {
                    if !after.contains(&number) {
                        correct = false;
                    }
                }
            }

            if correct {
                numbers[numbers.len() / 2]
            } else {
                0
            }
        })
        .sum();

    // Ok("".to_string())
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47";

        let result = process(input).unwrap();
        let answer = "143".to_string();
        assert_eq!(result, answer);
    }
}
