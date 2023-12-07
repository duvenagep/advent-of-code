struct Grid {
    x: i32,
    y: i32,
    touches: bool,
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("{:?}", _input);

    let grid: Vec<Vec<&str>> = _input
        .lines()
        .map(|line| line.split("").filter(|c| !c.is_empty()).collect())
        .collect();

    println!("{:?}", grid);

    let engine_parts: Vec<Grid> = Vec::new();
    let mut new_grid: Vec<Vec<&str>> = Vec::from(Vec::new());

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = process(input).unwrap();
        let answer = "4361".to_string();
        assert_eq!(result, answer);
    }
}
