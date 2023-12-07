enum fertilizer {
    Seed,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemp,
    TempToHumidity,
    HumidityToLocation,
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let seeds: Vec<i32> = _input
        .split_once("\n")
        .unwrap()
        .0
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let alamanac: Vec<&str> = _input.split("\n").collect();

    println!("{:?}", alamanac);
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";

        let result = process(input).unwrap();
        let answer = "35".to_string();
        assert_eq!(result, answer);
    }
}
