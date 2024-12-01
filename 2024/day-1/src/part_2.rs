pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    for line in _input.lines() {
        let mut iter = line.split_whitespace();
        list_1.push(iter.next().unwrap().parse::<i64>().unwrap());
        list_2.push(iter.next().unwrap().parse::<i64>().unwrap());
    }

    let v = list_1.iter().fold(0, |mut acc, elem| {
        let count = list_2.iter().filter(|e| e == &elem).count();
        acc += elem * count as i64;
        acc
    });

    Ok(v.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

        let result = process(input).unwrap();
        let answer = "31".to_string();
        assert_eq!(result, answer);
    }
}
