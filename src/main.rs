use std::error::Error;
use std::collections::HashMap;

mod fancyiters;
mod inputhandling;

fn main() -> Result<(), Box<dyn Error>> {
  let inputs = inputhandling::get_input_chars(8)?;

  let mut layers : HashMap<u32, HashMap<char, u32>> = HashMap::new();
  let mut layer = 0;

  for chunk in inputs.chunks(25*6) {
    let mut map : HashMap<char, u32> = HashMap::new();
    for c in chunk {
      let count = map.get(c).cloned().unwrap_or(0);
      map.insert(*c, count + 1);
    }

    layers.insert(layer, map);

    layer += 1;
  }

  let mut least_zeroes = 2000000000;

  for (k, v) in layers.iter() {
    if v.get(&'0').cloned().unwrap_or(2000000) < least_zeroes {
      layer = *k;
      least_zeroes = v.get(&'0').cloned().unwrap()
    }
  }

  let min_layer = layers.get(&layer).unwrap();

  println!("The numbers of ones times two is {}", min_layer.get(&'1').cloned().unwrap() * min_layer.get(&'2').cloned().unwrap());

  Ok(())
}

