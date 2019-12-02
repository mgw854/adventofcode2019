use std::error::Error;

mod inputhandling;
mod day1;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input : Vec<usize> = inputhandling::parse_csv_input(2, |s| s.parse::<usize>().map_err(|e| e.into()))?;
    
    input[1] = 12;
    input[2] = 2;

    let mut i = 0;

    while i < input.len() {
        match input[i] {
            1 => {
                let store = input[i + 3];
                let result = input[input[i + 1]] + input[input[i + 2]];
                input[store] = result;
            },
            2 => {
                let store = input[i + 3];
                let result = input[input[i + 1]] * input[input[i + 2]];
                input[store] = result;
            },
            99 => break,
            _ => continue
        }

        i += 4;
    }

    println!("The value at position 0 is {}", input[0]);
    
    Ok(())
}
