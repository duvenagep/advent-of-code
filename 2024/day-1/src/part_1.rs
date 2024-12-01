pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    for line in _input.lines() {
        let mut iter = line.split_whitespace();
        list_1.push(iter.next().unwrap().parse::<i64>().unwrap());
        list_2.push(iter.next().unwrap().parse::<i64>().unwrap());
    }

    list_1.sort();
    list_2.sort();

    println!("{:?}", list_1[0]);
    println!("{:?}", list_2[0]);

    let result: i64 = list_1
        .iter()
        .zip(list_2.iter())
        .map(|d| if d.0 > d.1 { d.0 - d.1 } else { d.1 - d.0 })
        .sum();

    Ok(result.to_string())
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
        let answer = "11".to_string();
        assert_eq!(result, answer);
    }
}
