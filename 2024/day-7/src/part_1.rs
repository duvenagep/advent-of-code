fn check(nums: &Vec<usize>, idx: usize, current: usize, test: &usize) -> bool {
    if idx >= nums.len() {
        return current == *test;
    }

    if check(nums, idx + 1, current + nums[idx], test) {
        return true;
    }
    if check(nums, idx + 1, current * nums[idx], test) {
        return true;
    }

    false
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let calibrations: Vec<(usize, Vec<usize>)> = _input
        .trim()
        .lines()
        .map(|line| {
            let (val, eq) = line.trim().split_once(":").unwrap();
            let value = val.parse::<usize>().unwrap();
            let eq: Vec<usize> = eq
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            (value, eq)
        })
        .collect();

    let mut result = 0;

    for (test, nums) in calibrations.iter() {
        if check(nums, 1, nums[0], test) {
            result += test;
        }
    }
    println!("{:?}", result);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20";

        let result = process(input).unwrap();
        let answer = "3749".to_string();
        assert_eq!(result, answer);
    }
}
