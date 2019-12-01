pub fn calculate_needed_fuel(mass: u32) -> u32
{
    let mut mass_fuel = (mass / 3) - 2;

    let mut extra_fuel = calculate_fuels_fuel(mass_fuel);

    while extra_fuel > 0
    {
        mass_fuel += extra_fuel;
        extra_fuel = calculate_fuels_fuel(extra_fuel);
    }

    mass_fuel
}

fn calculate_fuels_fuel(mass_of_fuel: u32) -> u32
{
    match mass_of_fuel / 3 {
        2 => 0,
        1 => 0,
        0 => 0,
        m => m - 2
    }
}