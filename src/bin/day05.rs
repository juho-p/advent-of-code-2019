extern crate aoc19;

use aoc19::intcode;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let prog = intcode::read_program_file("inputs/day05.txt")?;

    {
        let mut cpu = intcode::ComputerState::new(prog.clone(), vec![1]);
        cpu.run_until_finished()?;

        println!("part1: {:?}", cpu.output().last());
    }

    {
        let mut cpu = intcode::ComputerState::new(prog, vec![5]);
        cpu.run_until_finished()?;

        println!("part1: {:?}", cpu.output());
    }

    Ok(())
}
