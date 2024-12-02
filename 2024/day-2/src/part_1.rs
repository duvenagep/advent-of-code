pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lines = _input
        .lines()
        .map(|line| {
            let report: Vec<u32> = line
                .trim()
                .split_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .collect();

            let is_ascending = report.windows(2).all(|w| w[0] < w[1]);
            let is_descending = report.windows(2).all(|w| w[0] > w[1]);

            if !(is_ascending || is_descending) {
                return false;
            }

            let safe = report.windows(2).all(|w| {
                let diff = w[0].abs_diff(w[1]);
                diff >= 1 && diff <= 3
            });
            safe
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
        let answer = "2".to_string();
        assert_eq!(result, answer);
    }
}
