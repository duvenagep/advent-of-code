pub fn buffer_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = grid;
    for x in &mut new_grid {
        x.insert(0, '.');
        x.insert(x.len(), '.');
    }
    new_grid.insert(0, vec!['.'; new_grid.len()]);
    new_grid.insert(new_grid.len(), vec!['.'; new_grid.len()]);
    new_grid
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("{:?}", _input);

    let grid = _input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let input = buffer_grid(grid);

    let mut output = 0;
    let mut temp = String::new();

    for x in 1..input.len() - 1 {
        let mut is_part_number = false;
        for y in 1..input[x].len() - 1 {
            if input[x][y].is_ascii_digit() {
                temp.push(input[x][y]);

                if (input[x + 1][y] != '.' && !input[x + 1][y].is_ascii_digit())
                    || (input[x - 1][y] != '.' && !input[x - 1][y].is_ascii_digit())
                {
                    is_part_number = true;
                }

                if (input[x][y + 1] != '.' && !input[x][y + 1].is_ascii_digit())
                    || (input[x][y - 1] != '.' && !input[x][y - 1].is_ascii_digit())
                {
                    is_part_number = true;
                }

                if (input[x + 1][y + 1] != '.' && !input[x + 1][y + 1].is_ascii_digit())
                    || (input[x - 1][y - 1] != '.' && !input[x - 1][y - 1].is_ascii_digit())
                    || (input[x + 1][y - 1] != '.' && !input[x + 1][y - 1].is_ascii_digit())
                    || (input[x - 1][y + 1] != '.' && !input[x - 1][y + 1].is_ascii_digit())
                {
                    is_part_number = true;
                }
            } else {
                if is_part_number {
                    output += temp.parse::<i32>()?;
                }
                is_part_number = false;
                temp.clear();
                continue;
            }
        }
        if is_part_number {
            output += temp.parse::<i32>()?;
            temp.clear();
        }
    }
    println!("Output : {}", output);

    Ok(output.to_string())
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
