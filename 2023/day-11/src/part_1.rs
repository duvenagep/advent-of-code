#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    char: char,
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(solve(_input))
}

fn solve(input: &str) -> String {
    input
        .lines()
        .map(|l| l.trim())
        .flat_map(|line| {
            if line.contains("#") {
                return vec![line];
            }

            return vec![line, line];
        })
        .fold(vec![vec![]], |mut matrix, line| {
            for (char_index, char) in line.chars().enumerate() {
                match matrix.get_mut(char_index) {
                    Some(matrix_line) => matrix_line.push(char),
                    _ => matrix.push(vec![char]),
                }
            }

            matrix
        })
        .iter()
        .flat_map(|line| {
            if line.contains(&'#') {
                return vec![line];
            }

            return vec![line, line];
        })
        .enumerate()
        .map(|(line_index, chars)| {
            chars
                .iter()
                .enumerate()
                .map(|(char_index, char)| Node {
                    x: char_index,
                    y: line_index,
                    char: *char,
                })
                .collect::<Vec<Node>>()
        })
        .flat_map(|nodes| {
            nodes
                .into_iter()
                .filter(|n| n.char.ne(&'.'))
                .collect::<Vec<Node>>()
        })
        .fold(
            (vec![] as Vec<Node>, 0),
            |(mut galaxies, mut answer), node| {
                for galaxy in &galaxies {
                    let diff_x = galaxy.x.abs_diff(node.x);
                    let diff_y = galaxy.y.abs_diff(node.y);
                    answer += diff_x + diff_y;
                }

                galaxies.push(node);

                (galaxies, answer)
            },
        )
        .1
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        let result = process(input).unwrap();
        let answer = "374".to_string();
        assert_eq!(result, answer);
    }
}
