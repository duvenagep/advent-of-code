use std::collections::HashSet;

fn build_grid(input: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut start = (0, 0);
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.trim()
                .chars()
                .enumerate()
                .map(|(x, chr)| {
                    if chr == '^' {
                        start.0 = x as i32;
                        start.1 = y as i32;
                    }
                    chr
                })
                .collect()
        })
        .collect();
    (grid, start)
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // let (grid, start) = build_grid(_input);
    // for e in &grid {
    //     println!("{:?}", e);
    // }
    // let mut pos = start;
    // let mut dir = (0, -1);
    // let mut seen = HashSet::new();

    // loop {
    //     seen.insert(pos);

    //     let next = ((pos.0 + dir.0), (pos.1 + dir.1));
    //     let val = grid[next.0 as usize][next.1 as usize];

    //     if next.0 < 0 || next.0 == grid.len() as i32 || next.1 < 0 || next.1 == grid[0].len() as i32
    //     {
    //         break;
    //     }

    //     if grid[next.0 as usize][next.1 as usize] == '#' {
    //         dir = match dir {
    //             (-1, 0) => (0, 1),
    //             (0, 1) => (1, 0),
    //             (1, 0) => (0, -1),
    //             (0, -1) => (-1, 0),
    //             _ => panic!(),
    //         }
    //     } else {
    //         pos = next;
    //     }
    // }
    // let result = seen.len();
    // println!("{:?}", result);\
    let mut pos = (0, 0);
    let grid: Vec<Vec<char>> = _input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == '^' {
                        pos = (y as i32, x as i32);
                    };
                    char
                })
                .collect()
        })
        .collect();

    let m = grid.len();
    let n = grid[0].len();

    let mut dir = (-1, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    loop {
        visited.insert(pos);

        let next = ((pos.0 + dir.0), (pos.1 + dir.1));

        if next.0 < 0 || next.0 as usize == m || next.1 < 0 || next.1 as usize == n {
            break;
        }

        if grid[next.0 as usize][next.1 as usize] == '#' {
            dir = match dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            }
        } else {
            pos = next;
        }
    }

    let result = visited.len() as i32;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...";

        let result = process(input).unwrap();
        let answer = "41".to_string();
        assert_eq!(result, answer);
    }
}
