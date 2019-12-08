extern crate aoc19;

use aoc19::intcode::*;

fn run(init: Vec<Word>, a: Word, b: Word) -> Word {
    let mut computer = ComputerState::new(init, vec![]);
    computer.write(1, a).unwrap();
    computer.write(2, b).unwrap();

    while !computer.is_finished() {
        computer.step().unwrap();
    }

    computer.read(0).unwrap()
}

fn main() -> std::io::Result<()> {
    let values: Vec<_> = read_program_file("inputs/day02.txt")?;

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
