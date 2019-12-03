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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuels_fuel_6_to_0() {
        assert_eq!(calculate_fuels_fuel(6), 0);
    }

    #[test]
    fn test_calculate_fuels_fuel_3_to_0() {
        assert_eq!(calculate_fuels_fuel(3), 0);
    }

    #[test]
    fn test_calculate_fuels_fuel_1_to_0() {
        assert_eq!(calculate_fuels_fuel(1), 0);
    }

    
    #[test]
    fn test_calculate_fuels_fuel_15_to_5() {
        assert_eq!(calculate_fuels_fuel(15), 3);
    }
}