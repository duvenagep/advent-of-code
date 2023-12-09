struct Grid {
    x: i32,
    y: i32,
    touches: bool,
}

pub fn buffer_grid(grid: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    for x in grid.len() {
        for y in grid[x] {}
    }
    grid
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("{:?}", _input);

    let mut grid = _input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

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
