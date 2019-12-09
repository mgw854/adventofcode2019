use std::error::Error;
use std::collections::HashMap;

mod fancyiters;
mod inputhandling;
mod intcode_8086;

fn main() -> Result<(), Box<dyn Error>> {
  let inputs = inputhandling::get_input_chars(8)?;

  let mut layers : HashMap<u32, HashMap<u8, u32>> = HashMap::new();
  let mut layer = 1;

  for chunk in inputs.chunks(25*6) {
    let mut map : HashMap<u8, u32> = HashMap::new();
    for c in chunk {
      let cv = match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("Not supposed to get here")
      };
      let count = map.get(&cv).cloned().unwrap_or(0);
      map.insert(cv, count + 1);
    }

    layers.insert(layer, map);

    layer += 1;
  }

  use bmp::{ Image, Pixel };

  let mut image = Image::new(25, 6);

  for i in 0..25 as u32 {
    for j in 0..6 as u32 {
      for chunk in inputs.chunks(25*6) {
       match chunk.iter().skip((j as usize * 25) + i as usize).nth(0).unwrap() {
         '0' => { image.set_pixel(i, j, Pixel::new(0,0,0)); break; },
         '1' => { image.set_pixel(i, j, Pixel::new(255,255,255)); break; }
         '2' => continue,
         _ => panic!("Can't get here")
        }
      }
    }
  }
  
  let _ = image.save(".\\src\\day8\\img.bmp");

  Ok(())
}

