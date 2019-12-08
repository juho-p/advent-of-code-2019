extern crate aoc19;

use std::fs;
use aoc19::intcode::*;

fn run(init: Vec<Word>, a: Word, b: Word) -> Word {
    let mut computer = ComputerState::new(init);
    computer.write(1, a).unwrap();
    computer.write(2, b).unwrap();

    while !computer.is_finished() {
        computer.step().unwrap();
    }

    computer.read(0).unwrap()
}

fn main() -> std::io::Result<()> {
    let input_string = fs::read_to_string("inputs/day02.txt")?;

    let values: Vec<_> = input_string.trim_end().split(',')
        .flat_map(|token| {
            let parse_result = token.parse::<Word>();
            if parse_result.is_err() {
                println!("Invalid input `{}` {:?}", token, parse_result);
            }
            parse_result
        })
        .collect();

    let part1 = run(values.clone(), 12, 2);
    println!("part 1: {}", part1);

    for input in 0usize..100000usize {
        let noun = input / 100;
        let verb = input % 100;
        let output = run(values.clone(), noun as Word, verb as Word);
        if output == 19690720 {
            println!("part 2: {}", input);
            break;
        }
    }

    Ok(())
}
