extern crate aoc19;

use itertools::Itertools;

use aoc19::intcode;
use aoc19::intcode::Word;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn compute_signal(prog: &[Word], sequence: &[Word]) -> Word {
    sequence.iter().fold(0, |signal, &setting| {
        intcode::run_until_output(&mut intcode::State::new(prog.into(), vec![setting, signal])).unwrap().unwrap()
    })
}

fn compute_feedback_signal(prog: &[Word], sequence: &[Word]) -> Word {
    let mut amplifiers: Vec<intcode::State> = sequence.iter()
        .map(|&setting| intcode::State::new(prog.into(), vec![setting]))
        .collect();

    let mut signal = 0;

    loop {
        for amplifier in amplifiers.iter_mut() {
            amplifier.add_input(signal);
            signal = match intcode::run_until_output(amplifier).unwrap() {
                Some(x) => x,
                None => return signal,
            }
        }
    }
}

fn main() -> Result<()> {
    let program = intcode::read_program_file("inputs/day07.txt")?;

    let best_seq = (0..5).permutations(5).map(|seq| compute_signal(&program, &seq)).max();
    println!("{:?}", best_seq);

    let part2 = (5..10).permutations(5).map(|seq| compute_feedback_signal(&program, &seq)).max();
    println!("{:?}", part2);

    Ok(())
}
