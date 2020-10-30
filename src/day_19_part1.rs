/*
    --- Day 19: Go With The Flow ---
    With the Elves well on their way constructing the North Pole base, you turn your attention back to understanding the inner workings of programming the device.

    You can't help but notice that the device's opcodes don't contain any flow control like jump instructions. The device's manual goes on to explain:

    "In programs where flow control is required, the instruction pointer can be bound to a register so that it can be manipulated directly. This way, setr/seti can function as absolute jumps, addr/addi can function as relative jumps, and other opcodes can cause truly fascinating effects."

    This mechanism is achieved through a declaration like #ip 1, which would modify register 1 so that accesses to it let the program indirectly access the instruction pointer itself. To compensate for this kind of binding, there are now six registers (numbered 0 through 5); the five not bound to the instruction pointer behave as normal. Otherwise, the same rules apply as the last time you worked with this device.

    When the instruction pointer is bound to a register, its value is written to that register just before each instruction is executed, and the value of that register is written back to the instruction pointer immediately after each instruction finishes execution. Afterward, move to the next instruction by adding one to the instruction pointer, even if the value in the instruction pointer was just updated by an instruction. (Because of this, instructions must effectively set the instruction pointer to the instruction before the one they want executed next.)

    The instruction pointer is 0 during the first instruction, 1 during the second, and so on. If the instruction pointer ever causes the device to attempt to load an instruction outside the instructions defined in the program, the program instead immediately halts. The instruction pointer starts at 0.

    It turns out that this new information is already proving useful: the CPU in the device is not very powerful, and a background process is occupying most of its time. You dump the background process' declarations and instructions to a file (your puzzle input), making sure to use the names of the opcodes rather than the numbers.

    For example, suppose you have the following program:

    #ip 0
    seti 5 0 1
    seti 6 0 2
    addi 0 1 0
    addr 1 2 3
    setr 1 0 0
    seti 8 0 4
    seti 9 0 5
    When executed, the following instructions are executed. Each line contains the value of the instruction pointer at the time the instruction started, the values of the six registers before executing the instructions (in square brackets), the instruction itself, and the values of the six registers after executing the instruction (also in square brackets).

    ip=0 [0, 0, 0, 0, 0, 0] seti 5 0 1 [0, 5, 0, 0, 0, 0]
    ip=1 [1, 5, 0, 0, 0, 0] seti 6 0 2 [1, 5, 6, 0, 0, 0]
    ip=2 [2, 5, 6, 0, 0, 0] addi 0 1 0 [3, 5, 6, 0, 0, 0]
    ip=4 [4, 5, 6, 0, 0, 0] setr 1 0 0 [5, 5, 6, 0, 0, 0]
    ip=6 [6, 5, 6, 0, 0, 0] seti 9 0 5 [6, 5, 6, 0, 0, 9]
    In detail, when running this program, the following events occur:

    The first line (#ip 0) indicates that the instruction pointer should be bound to register 0 in this program. This is not an instruction, and so the value of the instruction pointer does not change during the processing of this line.
    The instruction pointer contains 0, and so the first instruction is executed (seti 5 0 1). It updates register 0 to the current instruction pointer value (0), sets register 1 to 5, sets the instruction pointer to the value of register 0 (which has no effect, as the instruction did not modify register 0), and then adds one to the instruction pointer.
    The instruction pointer contains 1, and so the second instruction, seti 6 0 2, is executed. This is very similar to the instruction before it: 6 is stored in register 2, and the instruction pointer is left with the value 2.
    The instruction pointer is 2, which points at the instruction addi 0 1 0. This is like a relative jump: the value of the instruction pointer, 2, is loaded into register 0. Then, addi finds the result of adding the value in register 0 and the value 1, storing the result, 3, back in register 0. Register 0 is then copied back to the instruction pointer, which will cause it to end up 1 larger than it would have otherwise and skip the next instruction (addr 1 2 3) entirely. Finally, 1 is added to the instruction pointer.
    The instruction pointer is 4, so the instruction setr 1 0 0 is run. This is like an absolute jump: it copies the value contained in register 1, 5, into register 0, which causes it to end up in the instruction pointer. The instruction pointer is then incremented, leaving it at 6.
    The instruction pointer is 6, so the instruction seti 9 0 5 stores 9 into register 5. The instruction pointer is incremented, causing it to point outside the program, and so the program ends.
    What value is left in register 0 when the background process halts?
*/

use regex::Regex;
use std::ops::{ Index, IndexMut };

const NUM_REGISTERS: usize = 6;

#[derive(Clone, Debug, Eq, PartialEq)]
struct State([u32; NUM_REGISTERS]);

impl Index<u32> for State {
    type Output = u32;

    fn index(&self, index: u32) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u32> for State {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl Index<usize> for State {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for State {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug)]
enum Error {
    InvalidOpcode,
    InvalidFunctioncode,
    InvalidInputA,
    InvalidInputB,
    InvalidOutputC,
}

struct Instruction {
    opcode: u8,
    input_a: u32,
    input_b: u32,
    output_c: u32,
}

impl Instruction {
    fn from_string(input: &str) -> Option<Self> {
        let mut split = input.trim().split(' ');

        let opcode = match split.next().unwrap() {
            "addr" => 0,
            "addi" => 1,
            "mulr" => 2,
            "muli" => 3,
            "banr" => 4,
            "bani" => 5,
            "borr" => 6,
            "bori" => 7,
            "setr" => 8,
            "seti" => 9,
            "gtir" => 10,
            "gtri" => 11,
            "gtrr" => 12,
            "eqir" => 13,
            "eqri" => 14,
            "eqrr" => 15,
            _  => return None,
        };

        let input_a = split.next().unwrap().parse::<u32>();
        if input_a.is_err() {
            return None;
        }

        let input_b = split.next().unwrap().parse::<u32>();
        if input_b.is_err() {
            return None;
        }

        let output_c = split.next().unwrap().parse::<u32>();
        if output_c.is_err() {
            return None;
        }

        Some(Self {
            opcode,
            input_a:  input_a.unwrap(),
            input_b:  input_b.unwrap(),
            output_c: output_c.unwrap(),
        })
    }

    fn many_from_string(input: &str) -> Vec<Self> {
        input.lines()
            .filter_map(Self::from_string)
            .collect()
    }

    fn validate_opcode(&self) -> Result<(), Error> {
        if self.opcode > 15 {
            Err(Error::InvalidOpcode)
        } else {
            Ok(())
        }
    }

    fn validate_reg_a(&self) -> Result<(), Error> {
        if self.input_a >= NUM_REGISTERS as u32 {
            Err(Error::InvalidInputA)
        } else {
            Ok(())
        }
    }

    fn validate_reg_b(&self) -> Result<(), Error> {
        if self.input_b >= NUM_REGISTERS as u32 {
            Err(Error::InvalidInputB)
        } else {
            Ok(())
        }
    }

    fn validate_reg_c(&self) -> Result<(), Error> {
        if self.output_c >= NUM_REGISTERS as u32 {
            Err(Error::InvalidOutputC)
        } else {
            Ok(())
        }
    }

    fn dispatch(&self, input: State) -> Result<State, Error> {
        self.validate_opcode()?;
        match self.opcode {
            0  => self.addr(input),
            1  => self.addi(input),
            2  => self.mulr(input),
            3  => self.muli(input),
            4  => self.banr(input),
            5  => self.bani(input),
            6  => self.borr(input),
            7  => self.bori(input),
            8  => self.setr(input),
            9  => self.seti(input),
            10 => self.gtir(input),
            11 => self.gtri(input),
            12 => self.gtrr(input),
            13 => self.eqir(input),
            14 => self.eqri(input),
            15 => self.eqrr(input),
            _  => Err(Error::InvalidFunctioncode),
        }
    }

    fn addr(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_b()?;
        self.validate_reg_c()?;

        let mut state = input;
        state[self.output_c] = state[self.input_a] + state[self.input_b];
        Ok(state)
    }

    fn addi(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_c()?;

        let mut state = input;
        state[self.output_c] = state[self.input_a] + self.input_b;
        Ok(state)
    }

    fn mulr(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_b()?;
        self.validate_reg_c()?;

        let mut state = input;
        state[self.output_c] = state[self.input_a] * state[self.input_b];
        Ok(state)
    }

    fn muli(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_c()?;

        let mut state = input;
        state[self.output_c] = state[self.input_a] * self.input_b;
        Ok(state)
    }

    fn banr(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_b()?;
        self.validate_reg_c()?;

        let mut state = input;
        state[self.output_c] = state[self.input_a] & state[self.input_b];
        Ok(state)
    }

    fn bani(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_c()?;

        let mut state = input;
        state[self.output_c] = state[self.input_a] & self.input_b;
        Ok(state)
    }

    fn borr(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_b()?;
        self.validate_reg_c()?;

        let mut state = input;
        state[self.output_c] = state[self.input_a] | state[self.input_b];
        Ok(state)
    }

    fn bori(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_c()?;

        let mut state = input;
        state[self.output_c] = state[self.input_a] | self.input_b;
        Ok(state)
    }

    fn setr(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_c()?;

        let mut state = input;
        state[self.output_c] = state[self.input_a];
        Ok(state)
    }

    fn seti(&self, input: State) -> Result<State, Error> {
        self.validate_reg_c()?;

        let mut state = input;
        state[self.output_c] = self.input_a;
        Ok(state)
    }

    fn gtir(&self, input: State) -> Result<State, Error> {
        self.validate_reg_b()?;
        self.validate_reg_c()?;

        let mut state = input;
        if self.input_a > state[self.input_b] {
            state[self.output_c] = 1;
        } else {
            state[self.output_c] = 0;
        }
        Ok(state)
    }

    fn gtri(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_c()?;

        let mut state = input;
        if state[self.input_a] > self.input_b {
            state[self.output_c] = 1;
        } else {
            state[self.output_c] = 0;
        }
        Ok(state)
    }

    fn gtrr(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_b()?;
        self.validate_reg_c()?;

        let mut state = input;
        if state[self.input_a] > state[self.input_b] {
            state[self.output_c] = 1;
        } else {
            state[self.output_c] = 0;
        }
        Ok(state)
    }

    fn eqir(&self, input: State) -> Result<State, Error> {
        self.validate_reg_b()?;
        self.validate_reg_c()?;

        let mut state = input;
        if self.input_a == state[self.input_b] {
            state[self.output_c] = 1;
        } else {
            state[self.output_c] = 0;
        }
        Ok(state)
    }

    fn eqri(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_c()?;

        let mut state = input;
        if state[self.input_a] == self.input_b {
            state[self.output_c] = 1;
        } else {
            state[self.output_c] = 0;
        }
        Ok(state)
    }

    fn eqrr(&self, input: State) -> Result<State, Error> {
        self.validate_reg_a()?;
        self.validate_reg_b()?;
        self.validate_reg_c()?;

        let mut state = input;
        if state[self.input_a] == state[self.input_b] {
            state[self.output_c] = 1;
        } else {
            state[self.output_c] = 0;
        }
        Ok(state)
    }
}

struct ChronalComputer {
    state: State,
    program: Vec<Instruction>,
    ip_reg: usize,
    ip: u32,
}

impl ChronalComputer {
    fn from_string(input: &str) -> Self {
        let ip_re = Regex::new(r"#ip (\d)").unwrap();
        let ip_caps = ip_re.captures(input).unwrap();
        let ip_reg = ip_caps[1].parse::<usize>().unwrap();
        assert!(ip_reg < NUM_REGISTERS);

        let program: Vec<Instruction> = Instruction::many_from_string(input);

        Self {
            state: State([0; NUM_REGISTERS]),
            program,
            ip_reg,
            ip: 0,
        }
    }

    fn run_program(&mut self) -> u32 {
        while (self.ip as usize) < self.program.len() {
            // Write the IP to its bound register
            self.state[self.ip_reg] = self.ip;

            // Execute the instruction
            self.state = self.program[self.ip as usize].dispatch(self.state.clone()).unwrap();

            // Write the register value back to the IP (plus an increment)
            self.ip = self.state[self.ip_reg] + 1;
        }

        self.state[0u32]
    }
}

#[aoc(day19, part1)]
pub fn solve(input: &str) -> u32 {
    let mut chronal = ChronalComputer::from_string(input);
    let result = chronal.run_program();
    println!("Register 0: {}", result);
    assert_eq!(result, 1302);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_program() {
        let input = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";
        let mut chronal = ChronalComputer::from_string(input);
        chronal.run_program();
        assert_eq!(chronal.state, State([6, 5, 6, 0, 0, 9]));
    }
}
