use std::io::{self, BufRead};

struct Module {
    mass: u32,
}

impl Module {
    fn calculate_fuel(&self) -> u32 {
        calculate_total_fuel(&self.mass)
    }
}

struct Mission {
    modules: Vec<Module>,
}

impl Mission {
    fn new() -> Mission {
        Mission {
            modules: vec![],
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut required_fuel_to_start_mission: u32 = 0;
    let mut mission: Mission = Mission::new();

    for line in stdin.lock().lines() {
        let module_mass = line.unwrap().parse::<u32>().unwrap();
        let module = Module {
            mass: module_mass,
        };

        required_fuel_to_start_mission += module.calculate_fuel();
    }

    eprintln!("Required fuel to initiate mission: {}", required_fuel_to_start_mission);
}

/// Fuel required to launch a given module is based on its mass.
/// Specifically, to find the fuel required for a module, take its mass,
/// divide by three, round down, and subtract 2.
fn calculate_fuel_for_mass(mass: &u32) -> i32 {
    ((*mass as f32 / 3.0).floor() - 2.0) as i32
}

/// Calculate fuel for modules and also include the fuel required
/// to carry fuel.
fn calculate_total_fuel(mass: &u32) -> u32 {
    let mut required_total_fuel: u32 = 0;
    let mut reduced_mass: u32 = *mass;

    loop {
        let required_fuel = calculate_fuel_for_mass(&reduced_mass);

        if required_fuel < 0 {
            break;
        } else {
            required_total_fuel += required_fuel as u32;
            reduced_mass = required_fuel as u32;
        }
    }

    required_total_fuel
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_calculate_fuel_requirements_to_move_mass() {
        assert_eq!(calculate_fuel_for_mass(&12), 2);
        assert_eq!(calculate_fuel_for_mass(&14), 2);
        assert_eq!(calculate_fuel_for_mass(&1969), 654);
    }

    #[test]
    fn it_can_calculate_fuel_for_mass_including_mass_of_fuel() {
        assert_eq!(calculate_total_fuel(&100756), 50346);
    }
}
