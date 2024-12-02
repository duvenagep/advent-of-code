pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lines = _input
        .lines()
        .map(|line| {
            let report: Vec<u32> = line
                .trim()
                .split_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .collect();

            let mut dampened = Vec::new();
            for i in 0..report.len() {
                let mut reduced = report.to_vec();
                reduced.remove(i);

                let ord = reduced[0].cmp(&reduced[1]);

                let mut safe = true;
                for e in reduced.windows(2) {
                    if e[0].cmp(&e[1]) != ord || e[0].abs_diff(e[1]) < 1 || e[0].abs_diff(e[1]) > 3
                    {
                        safe = false;
                        break;
                    }
                }
                dampened.push(safe);
            }
            dampened.iter().any(|b| *b)
        })
        .filter(|p| *p)
        .count();

    Ok(lines.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9";

        let result = process(input).unwrap();
        let answer = "4".to_string();
        assert_eq!(result, answer);
    }
}
