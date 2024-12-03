use regex::Regex;

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let re_mul = Regex::new(r"(?:don't\(\)|do\(\)|mul\(\d+,\d+\))").unwrap();

    let mul_list: Vec<&str> = re_mul.find_iter(&_input).map(|mul| mul.as_str()).collect();
    println!("{:?}", mul_list);
    let mut state = true;
    let mut result = 0;
    for element in mul_list {
        if element == "don't()" {
            state = false;
        }

        if state {
            let re_num = Regex::new(r"\d+,\d+").unwrap();
            if let Some(nums) = re_num.find(element) {
                let (x, y) = nums.as_str().split_once(",").unwrap();
                result += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap()
            };
        } else {
            state = false;
            if element == "do()" {
                state = true;
            }
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = process(input).unwrap();
        let answer = "48".to_string();
        assert_eq!(result, answer);
    }
}
