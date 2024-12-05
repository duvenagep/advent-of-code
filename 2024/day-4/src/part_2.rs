pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let grid: Vec<Vec<char>> = _input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i as usize][j as usize] == 'A' {
                if i - 1 >= 0 && i + 1 < m && j - 1 >= 0 && j + 1 < n {
                    let diag_1 = format!(
                        "{}{}{}",
                        grid[(i - 1) as usize][(j - 1) as usize],
                        grid[i as usize][j as usize],
                        grid[(i + 1) as usize][(j + 1) as usize]
                    );
                    let diag_2 = format!(
                        "{}{}{}",
                        grid[(i + 1) as usize][(j - 1) as usize],
                        grid[i as usize][j as usize],
                        grid[(i - 1) as usize][(j + 1) as usize]
                    );

                    if (diag_1 == "MAS" || diag_1 == "SAM") && (diag_2 == "MAS" || diag_2 == "SAM")
                    {
                        ans += 1;
                    }
                }
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
        let input = ".M.S......
        ..A..MSMS.
        .M.S.MAA..
        ..A.ASMSM.
        .M.S.M....
        ..........
        S.S.S.S.S.
        .A.A.A.A..
        M.M.M.M.M.
        ..........";

        let result = process(input).unwrap();
        let answer = "9".to_string();
        assert_eq!(result, answer);
    }
}
