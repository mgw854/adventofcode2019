use std::error::Error;

mod inputhandling;
mod intcode;
mod day1;

fn main() -> Result<(), Box<dyn Error>> {
    let mut nounverb = 0;

    let mut noun = 0;
    let mut verb = 0;

    while noun < 100
    {
        verb = 0;
        while verb < 100
        {

            let mut input : Vec<usize> = inputhandling::parse_csv_input(2, |s| s.parse::<usize>().map_err(|e| e.into()))?;
        
            input[1] = noun;
            input[2] = verb;
        
            let cpu = intcode::create(input);
    
            nounverb = cpu.process();
        
            if nounverb == 19690720 {
                println!("100*noun+verb = {}", 100 * noun + verb);
                return Ok(());
            }

            verb += 1;
        }

        noun += 1;
    }
    

    
    Ok(())
}
