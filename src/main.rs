use std::error::Error;

mod inputhandling;
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
        
            let mut i = 0;
        
            while i < input.len() {
                match input[i] {
                    1 => {
                        if i + 4 >= input.len() {
                            break;
                        }

                        let store = input[i + 3];

                        if (store >= input.len() || input[i + 1] >= input.len() || input[i+2] >= input.len()){
                            break;
                        }
                        let result = input[input[i + 1]] + input[input[i + 2]];
                        input[store] = result;
                    },
                    2 => {
                        if i + 4 >= input.len() {
                            break;
                        }

                        let store = input[i + 3];
                        if (store >= input.len() || input[i + 1] >= input.len() || input[i+2] >= input.len()){
                            break;
                        }
                        let result = input[input[i + 1]] * input[input[i + 2]];
                        input[store] = result;
                    },
                    99 => break,
                    _ => continue
                }
        
                i += 4;
            }
    
            nounverb = input[0];
        
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
