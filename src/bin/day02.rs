use std::fs;

type CpuResult<T> = Result<T, String>;

struct ComputerState {
    mem: Vec<usize>,
    pc: usize,
}

impl ComputerState {
    fn read(&self, i: usize) -> CpuResult<usize> {
        match self.mem.get(i) {
            Some(x) => Ok(*x),
            None => Err(format!("Invalid address: {}", i)),
        }
    }

    fn write(&mut self, i: usize, x: usize) -> CpuResult<()> {
        match self.mem.get_mut(i) {
            Some(r) => {
                *r = x;
                Ok(())
            },
            None => Err(format!("Invalid address: {}", i)),
        }
    }

    fn next_instruction(&self) -> CpuResult<usize> {
        self.read(self.pc)
    }

    fn read_operands(&self) -> CpuResult<(usize, usize, usize)> {
        let pc = self.pc;
        Ok((
            self.read(pc + 1)?,
            self.read(pc + 2)?,
            self.read(pc + 3)?,
        ))
    }

    fn advance(&mut self) {
        self.pc += 4;
    }

    fn is_finished(&self) -> bool {
        self.next_instruction() == Ok(99)
    }
}

fn exec(computer: &mut ComputerState) -> CpuResult<()> {
    let (a, b, c) = computer.read_operands()?;

    match computer.next_instruction()? {
        1 => computer.write(c, computer.read(a)? + computer.read(b)?),
        2 => computer.write(c, computer.read(a)? * computer.read(b)?),
        _ => Err("Invalid instruction".to_string()),
    }
}

fn run(init: Vec<usize>, a: usize, b: usize) -> usize {
    let mut computer = ComputerState {
        mem: init,
        pc: 0,
    };
    computer.mem[1] = a;
    computer.mem[2] = b;

    while !computer.is_finished() {
        exec(&mut computer).unwrap();
        computer.advance();
    }

    computer.mem[0]
}

fn main() -> std::io::Result<()> {
    let input_string = fs::read_to_string("inputs/day02.txt")?;

    let values: Vec<_> = input_string.trim_end().split(',')
        .flat_map(|token| {
            let parse_result = token.parse::<usize>();
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
        let output = run(values.clone(), noun, verb);
        if output == 19690720 {
            println!("part 2: {}", input);
            break;
        }
    }

    Ok(())
}
