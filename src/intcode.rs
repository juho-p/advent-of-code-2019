use std::fs;
use std::iter;

type CpuResult<T> = Result<T, String>;

pub type Word = i32;

#[derive(PartialEq)]
pub enum Status {
    Normal,
    Output(Word),
    Halted,
}

pub struct State {
    mem: Vec<Word>,
    pc: Word,
    done: bool,
    input: Vec<Word>,
}

impl State {
    pub fn new(mem: Vec<Word>, mut input: Vec<Word>) -> State {
        input.reverse();
        State {
            mem,
            pc: 0,
            done: false,
            input,
        }
    }

    pub fn read(&self, i: Word) -> CpuResult<Word> {
        match self.mem.get(i as usize) {
            Some(x) => Ok(*x),
            None => Err(format!("Invalid address: {}", i)),
        }
    }

    pub fn write(&mut self, i: Word, x: Word) -> CpuResult<()> {
        match self.mem.get_mut(i as usize) {
            Some(r) => {
                *r = x;
                Ok(())
            },
            None => Err(format!("Invalid address: {}", i)),
        }
    }

    pub fn step(&mut self) -> CpuResult<Status> {
        let opcode = self.read(self.pc)? as u32;
        let instruction = opcode % 100;
        let mode = opcode / 100;

        match instruction {
            1 => self.exec_arithmetic(mode, |a, b| a + b),
            2 => self.exec_arithmetic(mode, |a, b| a * b),
            3 => self.exec1(|m, a| {
                let input = m.read_input()?;
                m.write(a, input)
            }),
            4 => {
                let output = self.mode_val(mode, 0, self.read(self.pc + 1)?)?;
                self.pc += 2;
                Ok(Status::Output(output))
            },
            5 => self.exec_jump(mode, |a| a != 0),
            6 => self.exec_jump(mode, |a| a == 0),
            7 => self.exec_arithmetic(mode, |a, b| if a < b { 1 } else { 0 }),
            8 => self.exec_arithmetic(mode, |a, b| if a == b { 1 } else { 0 }),
            99 => {
                self.done = true;
                Ok(Status::Halted)
            }
            _ => Err(format!("Invalid opcode: {}", opcode)),
        }
    }

    fn read_input(&mut self) -> CpuResult<Word> {
        self.input.pop().ok_or_else(|| "Not enough input values".to_string())
    }

    pub fn add_input(&mut self, x: Word) {
        self.input.insert(0, x);
    }

    fn mode_val(&self, mode: u32, param: u32, x: Word) -> CpuResult<Word> {
        let param_mode =
            match param { 0 => mode, 1 => mode / 10, 2 => mode / 100, _ => panic!("invalid param") }
            % 10;

        if param_mode == 0 {
            self.read(x)
        } else {
            Ok(x)
        }
    }

    fn exec1(&mut self, f: impl Fn(&mut State, Word) -> CpuResult<()>) -> CpuResult<Status> {
        let pc = self.pc;
        self.pc += 2;
        f(self, self.read(pc + 1)?)?;
        Ok(Status::Normal)
    }

    fn exec3(&mut self, f: impl Fn(&mut State, Word, Word, Word) -> CpuResult<()>) -> CpuResult<Status> {
        let pc = self.pc;
        self.pc += 4;
        f(self, self.read(pc + 1)?, self.read(pc + 2)?,  self.read(pc + 3)?)?;
        Ok(Status::Normal)
    }

    fn exec_arithmetic(&mut self, mode: u32, f: impl Fn(Word, Word) -> Word) -> CpuResult<Status> {
        self.exec3(|m, a, b, c| m.write(
                c,
                f(m.mode_val(mode, 0, a)?, m.mode_val(mode, 1, b)?)))
    }

    fn exec_jump(&mut self, mode: u32, f: impl  Fn(Word) -> bool) -> CpuResult<Status> {
        if f(self.mode_val(mode, 0, self.read(self.pc + 1)?)?) {
            self.pc = self.mode_val(mode, 1, self.read(self.pc + 2)?)?;
        } else {
            self.pc += 3;
        }
        Ok(Status::Normal)
    }
}

pub fn run_until_output(state: &mut State) -> CpuResult<Option<Word>> {
    loop {
        match state.step()? {
            Status::Output(val) => return Ok(Some(val)),
            Status::Halted => return Ok(None),
            _ => ()
        }
    }
}

pub fn output_iter(mut state: State) -> impl iter::Iterator<Item = CpuResult<Word>> {
    iter::from_fn(move || {
        match run_until_output(&mut state) {
            Ok(Some(x)) => Some(Ok(x)),
            Ok(None) => None,
            Err(x) => Some(Err(x)),
        }
    }).take_while(|x| !x.is_err())
}

pub fn read_program_file(filename: &str) -> std::io::Result<Vec<Word>> {
    let input_string = fs::read_to_string(filename)?;

    let prog = input_string.trim_end().split(',')
        .flat_map(|token| {
            let parse_result = token.parse::<Word>();
            if parse_result.is_err() {
                println!("Invalid input `{}` {:?}", token, parse_result);
            }
            parse_result
        })
        .collect();

    Ok(prog)
}
