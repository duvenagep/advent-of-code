use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

use itertools::Itertools;

const ADJACENT_DELTA: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Hash)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn adjacent(&self) -> impl Iterator<Item = Position> + '_ {
        ADJACENT_DELTA.into_iter().filter_map(|(dx, dy)| {
            let new_x = self.x.checked_add_signed(dx);
            let new_y = self.y.checked_add_signed(dy);
            match (new_x, new_y) {
                (Some(x), Some(y)) => Some(Position { x, y }),
                _ => None,
            }
        })
    }

    fn is_open(&self, grid: &Vec<Vec<Pipe>>, curr_grid_pos: &Position) -> bool {
        use Pipe::*;
        // println!("New Position: {:?}", self);

        let current_pipe = grid[curr_grid_pos.x as usize][curr_grid_pos.y as usize];
        let new_pipe = grid[self.x as usize][self.y as usize];

        let allowed = match current_pipe {
            Start => {
                if &self.x < &curr_grid_pos.x {
                    match new_pipe {
                        Vertical | BottomRight | BottomLeft => true,
                        _ => false,
                    }
                } else if &self.x > &curr_grid_pos.x {
                    match new_pipe {
                        Vertical | TopRight | TopLeft => true,
                        _ => false,
                    }
                } else if &self.y < &curr_grid_pos.y {
                    match new_pipe {
                        Horizontal | BottomRight | TopRight => true,
                        _ => false,
                    }
                } else if &self.y > &curr_grid_pos.y {
                    match new_pipe {
                        Horizontal | BottomLeft | TopLeft => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            // match new_pipe {
            //     Vertical | Horizontal | TopRight | TopLeft | BottomRight | BottomLeft => true,
            //     _ => false,
            // },
            Vertical => {
                if &self.x < &curr_grid_pos.x {
                    match new_pipe {
                        Vertical | BottomLeft | BottomRight => true,
                        _ => false,
                    }
                } else if &self.x > &curr_grid_pos.x {
                    match new_pipe {
                        Vertical | TopRight | TopLeft => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            Horizontal => {
                if &self.y < &curr_grid_pos.y {
                    match new_pipe {
                        Horizontal | TopRight | BottomRight => true,
                        _ => false,
                    }
                } else if &self.y > &curr_grid_pos.y {
                    match new_pipe {
                        Horizontal | TopLeft | BottomLeft => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            TopLeft => {
                if &self.x < &curr_grid_pos.x {
                    match new_pipe {
                        Vertical | BottomLeft | BottomRight => true,
                        _ => false,
                    }
                } else if &self.y < &curr_grid_pos.y {
                    match new_pipe {
                        Horizontal | TopRight | BottomRight => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            TopRight => {
                if &self.x < &curr_grid_pos.x {
                    match new_pipe {
                        Vertical | BottomLeft | BottomRight => true,
                        _ => false,
                    }
                } else if &self.y > &curr_grid_pos.y {
                    match new_pipe {
                        Horizontal | TopLeft | BottomLeft => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            BottomLeft => {
                if &self.x > &curr_grid_pos.x {
                    match new_pipe {
                        Vertical | TopLeft | TopRight => true,
                        _ => false,
                    }
                } else if &self.y < &curr_grid_pos.y {
                    match new_pipe {
                        Horizontal | BottomRight | TopRight => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            BottomRight => {
                if &self.x > &curr_grid_pos.x {
                    match new_pipe {
                        Vertical | TopRight | TopLeft => true,
                        _ => false,
                    }
                } else if &self.y > &curr_grid_pos.y {
                    match new_pipe {
                        Horizontal | BottomLeft | TopLeft => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            Ground => false,
        };
        // println!(
        //     "from: {:?}, to: {:?} - {:?} -> {:?} : {:?}",
        //     curr_grid_pos, self, current_pipe, new_pipe, allowed
        // );
        allowed
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    Ground,
    Start,
}

fn build_grid(input: &str) -> (Vec<Vec<Pipe>>, Position) {
    let mut start: Position = Position::default();
    let grid = input
        .lines()
        .enumerate()
        .map(|(x, row)| {
            row.trim()
                .chars()
                .enumerate()
                .map(|(y, c)| {
                    let pipe = match c {
                        '|' => Pipe::Vertical,
                        '-' => Pipe::Horizontal,
                        'L' => Pipe::TopRight,
                        'J' => Pipe::TopLeft,
                        'F' => Pipe::BottomRight,
                        '7' => Pipe::BottomLeft,
                        '.' => Pipe::Ground,
                        'S' => Pipe::Start,
                        _ => unreachable!("No other grid characters"),
                    };

                    if pipe == Pipe::Start {
                        start.x = x as u32;
                        start.y = y as u32;
                    }
                    pipe
                })
                .collect()
        })
        .collect();
    (grid, start)
}

fn create_path(parents: HashMap<Position, Position>, goal: Position) -> Vec<Position> {
    let mut path = vec![];
    let mut curr = goal;

    while let Some(&parent) = parents.get(&curr) {
        path.push(parent);
        curr = parent
    }

    path.reverse();
    path
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let now = Instant::now();
    let (grid, start) = build_grid(_input);

    let mut parents: HashMap<Position, Position> = HashMap::new();
    let mut queue = VecDeque::from(vec![start]);
    let mut seen = HashSet::new();
    seen.insert(start);

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        for next in curr
            .adjacent()
            .filter(|el| {
                el.x < grid.len().try_into().unwrap() && el.y < grid[0].len().try_into().unwrap()
            })
            .filter(|next| next.is_open(&grid, &curr))
        {
            if !seen.contains(&next) {
                seen.insert(next);
                queue.push_back(next);
                parents.insert(curr, next);
            }
        }
    }

    let path = create_path(parents, start);
    let result = (path.len()) + 1;
    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let input = "7-F7-
        // .FJ|7
        // SJLL7
        // |F--J
        // LJ.LJ";

        let input = "-L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF";

        let result = process(input).unwrap();
        let answer = "8".to_string();
        assert_eq!(result, answer);
    }
}
