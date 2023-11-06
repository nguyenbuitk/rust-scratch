#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();
    // unwrap(): if error, terminate the program
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    println!("Input: {}", input);

    let mars_weight = calculate_weight_on_mars(100.0);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}