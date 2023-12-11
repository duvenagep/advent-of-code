use itertools::Itertools;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub fn build_map(input: &str) -> (Vec<u64>, Vec<(&str, Vec<(u64, u64, u64)>)>) {
    let (seeds, map) = input.split_once("\n").unwrap();
    let seeds: Vec<u64> = seeds
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .collect();

    let map: Vec<(&str, Vec<(u64, u64, u64)>)> = map
        .split("\n\n")
        .map(|chunk| {
            let x = chunk.trim();
            let (cat, ranges) = x.split_once("\n").unwrap();
            let ranges: Vec<(u64, u64, u64)> = ranges
                .lines()
                .map(|el| {
                    el.split_whitespace()
                        .map(|el| el.parse::<u64>().unwrap())
                        .collect_tuple::<(u64, u64, u64)>()
                        .unwrap()
                })
                .collect();
            (cat, ranges)
        })
        .collect();

    (seeds, map)
}

pub fn get_pos(point: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for triplet in map {
        if point >= triplet.1 && point < triplet.1 + triplet.2 {
            return triplet.0 + (point - triplet.1);
        }
    }
    point
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let now = Instant::now();
    let (seeds, routes) = build_map(_input);

    let calculated_locations = Arc::new(Mutex::new(u64::MAX));
    let mut seed_iter = seeds.iter();
    while let Some(start) = seed_iter.next() {
        if let Some(range) = seed_iter.next() {
            println!("looping...");
            (*start..*start + *range).into_par_iter().for_each(|x| {
                let soil = get_pos(x, &routes[0].1);
                let fertilizer = get_pos(soil, &routes[1].1);
                let water = get_pos(fertilizer, &routes[2].1);
                let light = get_pos(water, &routes[3].1);
                let temp = get_pos(light, &routes[4].1);
                let humidity = get_pos(temp, &routes[5].1);
                let location = get_pos(humidity, &routes[6].1);
                if *calculated_locations.lock().unwrap() > location {
                    println!("Maybe: {}", location);
                    *calculated_locations.lock().unwrap() = location;
                }
            });
        }
    }

    let result = calculated_locations.lock().unwrap();
    println!("{result}");
    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");

    Ok(result.to_string())
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
        let answer = "46".to_string();
        assert_eq!(result, answer);
    }
}
