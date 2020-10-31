/*
    --- Day 16: Chronal Classification ---
    As you see the Elves defend their hot chocolate successfully, you go back to falling through time. This is going to become a problem.

    If you're ever going to return to your own time, you need to understand how this device on your wrist works. You have a little while before you reach your next destination, and with a bit of trial and error, you manage to pull up a programming manual on the device's tiny screen.

    According to the manual, the device has four registers (numbered 0 through 3) that can be manipulated by instructions containing one of 16 opcodes. The registers start with the value 0.

    Every instruction consists of four values: an opcode, two inputs (named A and B), and an output (named C), in that order. The opcode specifies the behavior of the instruction and how the inputs are interpreted. The output, C, is always treated as a register.

    In the opcode descriptions below, if something says "value A", it means to take the number given as A literally. (This is also called an "immediate" value.) If something says "register A", it means to use the number given as A to read from (or write to) the register with that number. So, if the opcode addi adds register A and value B, storing the result in register C, and the instruction addi 0 7 3 is encountered, it would add 7 to the value contained by register 0 and store the sum in register 3, never modifying registers 0, 1, or 2 in the process.

    Many opcodes are similar except for how they interpret their arguments. The opcodes fall into seven general categories:

    Addition:

    addr (add register) stores into register C the result of adding register A and register B.
    addi (add immediate) stores into register C the result of adding register A and value B.
    Multiplication:

    mulr (multiply register) stores into register C the result of multiplying register A and register B.
    muli (multiply immediate) stores into register C the result of multiplying register A and value B.
    Bitwise AND:

    banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
    Bitwise OR:

    borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
    Assignment:

    setr (set register) copies the contents of register A into register C. (Input B is ignored.)
    seti (set immediate) stores value A into register C. (Input B is ignored.)
    Greater-than testing:

    gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    Equality testing:

    eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
    Unfortunately, while the manual gives the name of each opcode, it doesn't seem to indicate the number. However, you can monitor the CPU to see the contents of the registers before and after instructions are executed to try to work them out. Each opcode has a number from 0 through 15, but the manual doesn't say which is which. For example, suppose you capture the following sample:

    Before: [3, 2, 1, 1]
    9 2 1 2
    After:  [3, 2, 2, 1]
    This sample shows the effect of the instruction 9 2 1 2 on the registers. Before the instruction is executed, register 0 has value 3, register 1 has value 2, and registers 2 and 3 have value 1. After the instruction is executed, register 2's value becomes 2.

    The instruction itself, 9 2 1 2, means that opcode 9 was executed with A=2, B=1, and C=2. Opcode 9 could be any of the 16 opcodes listed above, but only three of them behave in a way that would cause the result shown in the sample:

    Opcode 9 could be mulr: register 2 (which has a value of 1) times register 1 (which has a value of 2) produces 2, which matches the value stored in the output register, register 2.
    Opcode 9 could be addi: register 2 (which has a value of 1) plus value 1 produces 2, which matches the value stored in the output register, register 2.
    Opcode 9 could be seti: value 2 matches the value stored in the output register, register 2; the number given for B is irrelevant.
    None of the other opcodes produce the result captured in the sample. Because of this, the sample above behaves like three opcodes.

    You collect many of these samples (the first section of your puzzle input). The manual also includes a small test program (the second section of your puzzle input) - you can ignore it for now.

    Ignoring the opcode numbers, how many samples in your puzzle input behave like three or more opcodes?
*/

use regex::Regex;
use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq)]
struct State([u32; 4]);

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
    fn from_string(input: &str) -> Self {
        let mut split = input.trim().split(' ');

        Self {
            opcode: split.next().unwrap().parse::<u8>().unwrap(),
            input_a: split.next().unwrap().parse::<u32>().unwrap(),
            input_b: split.next().unwrap().parse::<u32>().unwrap(),
            output_c: split.next().unwrap().parse::<u32>().unwrap(),
        }
    }

    fn validate_opcode(&self) -> Result<(), Error> {
        if self.opcode > 15 {
            Err(Error::InvalidOpcode)
        } else {
            Ok(())
        }
    }

    fn validate_reg_a(&self) -> Result<(), Error> {
        if self.input_a > 3 {
            Err(Error::InvalidInputA)
        } else {
            Ok(())
        }
    }

    fn validate_reg_b(&self) -> Result<(), Error> {
        if self.input_b > 3 {
            Err(Error::InvalidInputB)
        } else {
            Ok(())
        }
    }

    fn validate_reg_c(&self) -> Result<(), Error> {
        if self.output_c > 3 {
            Err(Error::InvalidOutputC)
        } else {
            Ok(())
        }
    }

    fn dispatch(&self, input: State, opcode_lookup: &[u8; 16]) -> Result<State, Error> {
        self.validate_opcode()?;
        let function_code = opcode_lookup[self.opcode as usize];
        self.dispatch_as(input, function_code)
    }

    fn dispatch_as(&self, input: State, function_code: u8) -> Result<State, Error> {
        match function_code {
            0 => self.addr(input),
            1 => self.addi(input),
            2 => self.mulr(input),
            3 => self.muli(input),
            4 => self.banr(input),
            5 => self.bani(input),
            6 => self.borr(input),
            7 => self.bori(input),
            8 => self.setr(input),
            9 => self.seti(input),
            10 => self.gtir(input),
            11 => self.gtri(input),
            12 => self.gtrr(input),
            13 => self.eqir(input),
            14 => self.eqri(input),
            15 => self.eqrr(input),
            _ => Err(Error::InvalidFunctioncode),
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
        self.validate_reg_a()?;
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

struct Sample {
    before: State,
    op: Instruction,
    after: State,
}

impl Sample {
    fn many_from_string(input: &str) -> Vec<Self> {
        let re = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]\n((?:\d+\s*)+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
        re.captures_iter(input)
            .map(|cap| Self {
                before: State([
                    cap[1].parse::<u32>().unwrap(),
                    cap[2].parse::<u32>().unwrap(),
                    cap[3].parse::<u32>().unwrap(),
                    cap[4].parse::<u32>().unwrap(),
                ]),
                op: Instruction::from_string(&cap[5]),
                after: State([
                    cap[6].parse::<u32>().unwrap(),
                    cap[7].parse::<u32>().unwrap(),
                    cap[8].parse::<u32>().unwrap(),
                    cap[9].parse::<u32>().unwrap(),
                ]),
            })
            .collect()
    }

    fn find_possible_opcodes(&self) -> Vec<u8> {
        // Returns a vec of all possible function codes
        (0..16)
            .map(|i| (i, self.op.dispatch_as(self.before.clone(), i)))
            .filter(|(_i, result)| match result {
                Ok(x) => x == &self.after,
                _ => false,
            })
            .map(|(i, _result)| i)
            .collect()
    }
}

fn count_ambiguous_opcodes(samples: &[Sample]) -> u32 {
    samples
        .iter()
        .map(|sample| sample.find_possible_opcodes())
        .filter(|func_codes| func_codes.len() >= 3)
        .count() as u32
}

#[aoc(day16, part1)]
pub fn solve(input: &str) -> u32 {
    let samples = Sample::many_from_string(input);
    let gt_3_opcodes_count = count_ambiguous_opcodes(&samples);
    println!("Samples >= 3 opcodes: {}", gt_3_opcodes_count);
    assert_eq!(gt_3_opcodes_count, 646);
    gt_3_opcodes_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dispatch_as() {
        let instr = Instruction::from_string("9 2 1 2");
        let before = State([3, 2, 1, 1]);
        let after = instr.dispatch_as(before, 2).unwrap();
        assert_eq!(after.0, [3, 2, 2, 1]);
    }

    #[test]
    fn test_try_all_opcodes() {
        let input = "
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";
        let samples = Sample::many_from_string(input);
        let possible_opcodes = samples[0].find_possible_opcodes();
        assert_eq!(possible_opcodes, vec![1, 2, 9]); // addi, mulr, seti
    }
}
