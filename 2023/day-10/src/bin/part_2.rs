use day_10::part_2::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = include_str!("../../input_2.txt");
    let result = process(file);
    println!("{:?}", result);
    Ok(())
}
