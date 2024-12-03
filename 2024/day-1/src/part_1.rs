pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    for line in _input.lines() {
        let (a, b) = line.split_once("   ").unwrap();
        list_1.push(a.parse::<u32>().unwrap());
        list_2.push(b.parse::<u32>().unwrap());
    }

    list_1.sort();
    list_2.sort();

    let result: u32 = list_1
        .iter()
        .zip(list_2.iter())
        .map(|d| d.0.abs_diff(*d.1))
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
