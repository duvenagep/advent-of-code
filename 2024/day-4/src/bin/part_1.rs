use day_4::part_1::process;
use std::time::Instant;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = include_str!("../../input_1.txt");
    let start: Instant = Instant::now();
    let result = process(file);
    let formatted_result = format!(
        "{} --- Elapsed time: {:.4?}",
        result.unwrap(),
        start.elapsed()
    );
    println!("{formatted_result}");
    Ok(())
}
