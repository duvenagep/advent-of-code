use std::collections::HashMap;

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
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for x in 1..input.len() - 1 {
        let mut is_part_number = false;
        let mut gear_found_loc = (0, 0);
        for y in 1..input[x].len() - 1 {
            if input[x][y].is_ascii_digit() {
                temp.push(input[x][y]);

                if input[x + 1][y] != '.' && !input[x + 1][y].is_ascii_digit() {
                    is_part_number = true;
                    if input[x + 1][y] == '*' {
                        gear_found_loc = (x + 1, y);
                    }
                }
                if input[x - 1][y] != '.' && !input[x - 1][y].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x - 1, y);
                }

                if input[x][y + 1] != '.' && !input[x][y + 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x, y + 1);
                }
                if input[x][y - 1] != '.' && !input[x][y - 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x, y - 1);
                }

                if input[x + 1][y + 1] != '.' && !input[x + 1][y + 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x + 1, y + 1);
                }

                if input[x - 1][y - 1] != '.' && !input[x - 1][y - 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x - 1, y - 1);
                }
                if input[x + 1][y - 1] != '.' && !input[x + 1][y - 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x + 1, y - 1);
                }
                if input[x - 1][y + 1] != '.' && !input[x - 1][y + 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x - 1, y + 1);
                }
            } else {
                if is_part_number && gear_found_loc != (0, 0) {
                    gears.entry(gear_found_loc).or_insert_with(Vec::new);
                    gears
                        .get_mut(&gear_found_loc)
                        .unwrap()
                        .push(temp.parse::<i32>()?);
                    gear_found_loc = (0, 0);
                }
                is_part_number = false;
                temp.clear();
                continue;
            }
        }
        if is_part_number {
            if gear_found_loc != (0, 0) {
                gears.entry(gear_found_loc).or_insert_with(Vec::new);
                gears
                    .get_mut(&gear_found_loc)
                    .unwrap()
                    .push(temp.parse::<i32>()?);
            }
            temp.clear();
        }
    }

    for v in gears.values() {
        if v.len() == 2 {
            output += v[0] * v[1];
        }
    }
    println!("Output : {:?}", output);

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
        let answer = "467835".to_string();
        assert_eq!(result, answer);
    }
}
