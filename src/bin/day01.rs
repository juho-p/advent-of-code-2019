use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn fuel_required(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    cmp::max(fuel, 0)
}

fn improved_fuel_required(mass: i32)  -> i32 {
    let fuel = fuel_required(mass);

    if fuel == 0 {
        fuel
    } else {
        fuel + improved_fuel_required(fuel)
    }
}

fn fuel_sum(masses: &Vec<i32>, f: fn(i32) -> i32) -> i32 {
    masses.iter()
        .cloned()
        .map(f)
        .sum()
}

fn part1(masses: &Vec<i32>) -> i32 {
    fuel_sum(masses, fuel_required)
}

fn part2(masses: &Vec<i32>) -> i32 {
    fuel_sum(masses, improved_fuel_required)
}

fn main() -> Result<()> {
    let file = File::open("inputs/day01.txt")?;

    let try_masses: Result<Vec<i32>> = BufReader::new(file)
        .lines()
        .map(|s| Ok(s?.parse::<i32>()?))
        .collect();

    let masses = try_masses?;

    println!("{}\n{}", part1(&masses), part2(&masses));

    Ok(())
}
