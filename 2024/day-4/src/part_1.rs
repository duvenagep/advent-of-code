use regex::Regex;

const ADJACENT_DELTA: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Pos {
    x: u32,
    y: u32,
}

// impl Pos {
//     fn adjacent(&self) -> impl Iterator<Item = Pos> + '_ {
//         (0..4).into_iter().flat_map(|n| {
//             ADJACENT_DELTA.into_iter().filter_map(|(dx, dy)| {
//                 let new_x = self.x.checked_add_signed(dx*n);
//                 let new_y = self.y.checked_add_signed(dy);
//                 match (new_x, new_y) {
//                     (Some(x), Some(y)) => Some(Pos { x, y }),
//                     _ => None,
//                 }
//             })
//         }

//         })
// }

impl Grid {
    fn new(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|line| line.trim().chars().collect())
                .collect(),
        }
    }
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pos: Vec<(i32, i32)> = (0..4)
        .into_iter()
        .flat_map(|n| {
            ADJACENT_DELTA
                .into_iter()
                .map(|(x, y)| (x * n, y * n))
                .collect::<Vec<(i32, i32)>>()
        })
        .filter(|t| *t != (0, 0))
        .collect();

    println!("{:?}", pos);
    // let mut grid = Grid::new(_input);

    // println!("{:?}", grid);

    // // let h = grid.len() as i32;
    // // let l = grid[0].len() as i32;

    // let mut ans = 0;

    // // let re =
    // //     Regex::new(r"(?m)XMAS|SAMX|X\nM\nA\nS|S\nA\nM\nX|^X\n\s*M\n\s*\S*A\n\s*\S*\S*S$").unwrap();
    // // let captures: Vec<&str> = re.find_iter(&_input).map(|mul| mul.as_str()).collect();
    // // println!("{:?}", captures);
    let grid: Vec<Vec<char>> = _input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i as usize][j as usize] == 'X' {
                ADJACENT_DELTA.iter().for_each(|(x, y)| {
                    let possibility = (0..4)
                        .filter_map(|c| {
                            if (i + x * c) >= 0
                                && (i + x * c) < m
                                && (j + y * c) >= 0
                                && (j + y * c) < n
                            {
                                Some(grid[(i + x * c) as usize][(j + y * c) as usize])
                            } else {
                                None
                            }
                        })
                        .collect::<String>();

                    if &possibility == "XMAS" {
                        ans += 1;
                    }
                });
            }
        }
    }
    Ok(ans.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";

        let result = process(input).unwrap();
        let answer = "181".to_string();
        assert_eq!(result, answer);
    }
}
