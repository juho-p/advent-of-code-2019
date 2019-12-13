extern crate aoc19;

use aoc19::intcode;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let prog = intcode::read_program_file("inputs/day05.txt")?;

    {
        let state = intcode::State::new(prog.clone(), vec![1]);
        let x = intcode::output_iter(state).last().ok_or("no output")??;

        println!("part1: {:?}", x);
    }

    {
        let output = intcode::run_until_output(&mut intcode::State::new(prog, vec![5]))?;

        println!("part2: {:?}", output);
    }

    Ok(())
}
