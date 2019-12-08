type CpuResult<T> = Result<T, String>;

pub type Word = u32;

pub struct ComputerState {
    mem: Vec<Word>,
    pc: Word,
    done: bool,
}

impl ComputerState {
    pub fn new(mem: Vec<Word>) -> ComputerState {
        ComputerState {
            mem,
            pc: 0,
            done: false
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

    pub fn step(&mut self) -> CpuResult<()> {
        let opcode = self.read(self.pc)?;
        match opcode {
            1 => self.exec3(|m, a, b, c| m.write(c, m.read(a)? + m.read(b)?)),
            2 => self.exec3(|m, a, b, c| m.write(c, m.read(a)? * m.read(b)?)),
            99 => {
                self.done = true;
                Ok(())
            }
            _ => Err(format!("Invalid opcode: {}", opcode)),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.done
    }

    fn exec3(&mut self, f: impl Fn(&mut ComputerState, Word, Word, Word) -> CpuResult<()>) -> CpuResult<()> {
        let pc = self.pc;
        f(self, self.read(pc + 1)?, self.read(pc + 2)?,  self.read(pc + 3)?)?;
        self.pc += 4;
        Ok(())
    }
}
