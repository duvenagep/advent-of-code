// gets the difference betweeen a seq in a vec
pub fn get_common_diff(seq: &Vec<i64>) -> Vec<i64> {
    seq.windows(2).map(|w| w[1] - w[0]).collect()
}

//checks every element == 0
pub fn is_zero(seq: &Vec<i64>) -> bool {
    seq.iter().all(|el| *el == 0)
}

pub fn generate_prev_term(seq: &Vec<i64>) -> i64 {
    if is_zero(seq) {
        return 0;
    }

    let prev_term = generate_prev_term(&get_common_diff(seq));

    seq.first().unwrap() - prev_term
}
pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let seq: Vec<Vec<i64>> = _input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|i| i.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let mut result = 0;

    seq.iter().for_each(|s| {
        result += generate_prev_term(&s);
    });

    println!("{:?}", result);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        let result = process(input).unwrap();
        let answer = "2".to_string();
        assert_eq!(result, answer);
    }
}
