use regex::Regex;

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let re_mul = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let mul_list: Vec<&str> = re_mul.find_iter(&_input).map(|mul| mul.as_str()).collect();
    let re_num = Regex::new(r"\d+,\d+").unwrap();
    let nums: Vec<Vec<(i32, i32)>> = mul_list
        .iter()
        .map(|elem| {
            let nums: Vec<&str> = re_num.find_iter(elem).map(|e| e.as_str()).collect();
            let mul = nums
                .iter()
                .map(|e| {
                    let (x, y) = e.split_once(",").unwrap();
                    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                })
                .collect::<Vec<(i32, i32)>>();
            mul
        })
        .collect();

    let result: i32 = nums.iter().flat_map(|e| e.iter().map(|f| f.0 * f.1)).sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = process(input).unwrap();
        let answer = "161".to_string();
        assert_eq!(result, answer);
    }
}
