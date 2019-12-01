use std::error::Error;

mod inputhandling;
mod day1;

fn main() -> Result<(), Box<Error>> {
    let input : Vec<u32> = inputhandling::parse_input(1, |s| s.parse::<u32>().map_err(|e| e.into()))?;
    
    let sum : u32 = input.iter().map(|mass| day1::calculate_needed_fuel(*mass)).sum();

    println!("The total fuel needed is {}", sum);
    
    Ok(())
}
