use std::fs;

fn main() {
    println!("Welcome to the first day of Advent of Code 2019!");
    let input = read_input_file("./resources/input_puzzel_1.txt");
    // println!("Input is : \n{}", input);
    let array = input_to_array(&input);
    let needed_fuel = calc_total_fuel(array);
    println!(
        "The needed fuel for all the elf's spacecrafts = {}",
        needed_fuel
    )
}

fn read_input_file(path: &str) -> String {
    let error = format!("Could not read file from path: {}", path);
    let content = fs::read_to_string(path).expect(error.as_ref());
    return content;
}

fn input_to_array(input: &str) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();
    for line in input.split("\n") {
        if let Ok(value) = line.parse() {
            list.push(value)
        }
    }
    list
}

fn calc_fuel_for_fuel(mass: i32) -> i32 {
    // Calculate the amount of fuel needed for a spacecraft,
    // with the weight of the fuel itself in mind.
    let mut fuel = (mass / 3) - 2;
    let mut fuel_for_last_fuel: i32 = fuel;
    while fuel_for_last_fuel > 0 {
        fuel_for_last_fuel = (fuel_for_last_fuel / 3) - 2;
        if fuel_for_last_fuel > 0 {
            fuel += fuel_for_last_fuel;
        }
    }
    fuel
}

fn calc_total_fuel(list_of_mass: Vec<i32>) -> i32 {
    let mut fuel_sum: i32 = 0;
    for mass in list_of_mass {
        fuel_sum += calc_fuel_for_fuel(mass);
    }
    fuel_sum
}
