use std::{self, io, process::exit};

fn parse_input(input_str: String) -> f32 {
    let parsed_input: f32 = input_str.trim().parse().expect("Error parsing input");
    return parsed_input;
}
fn read_input() -> String {
    let mut input_value = String::new();
    io::stdin().read_line(&mut input_value).expect("msg");
    return input_value;
}
fn main() {
    println!("input:");
    let speed = read_input();
    let speed_int = parse_input(&speed);
    let distance = 10.0; // Example distance in kilometers
    let speed = 60.0; // Example speed in kilometers per hour

    let time_in_hours = distance / speed; // Calculate the time in hours
    let time_in_seconds = time_in_hours * 3600.0; // Convert hours to seconds
    let time_in_minutes = time_in_seconds / 60.0;

    println!("Time: {} seconds", time_in_seconds);
    println!("Time: {} minutes", time_in_minutes);
}
