/*
    --- Part Two ---
    In order to determine the timing window for your underflow exploit, you also need an upper bound:

    What is the lowest non-negative integer value for register 0 that causes the program to halt after executing the most instructions? (The program must actually halt; running forever does not count as halting.)
*/

use regex::Regex;
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

const NUM_REGISTERS: usize = 6;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State([u64; NUM_REGISTERS]);

impl Index<u64> for State {
    type Output = u64;

    fn index(&self, index: u64) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u64> for State {
    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl Index<usize> for State {
    type Output = u64;

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
    input_a: u64,
    input_b: u64,
    output_c: u64,
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
            _ => return None,
        };

        let input_a = split.next().unwrap().parse::<u64>();
        if input_a.is_err() {
            return None;
        }

        let input_b = split.next().unwrap().parse::<u64>();
        if input_b.is_err() {
            return None;
        }

        let output_c = split.next().unwrap().parse::<u64>();
        if output_c.is_err() {
            return None;
        }

        Some(Self {
            opcode,
            input_a: input_a.unwrap(),
            input_b: input_b.unwrap(),
            output_c: output_c.unwrap(),
        })
    }

    fn many_from_string(input: &str) -> Vec<Self> {
        input.lines().filter_map(Self::from_string).collect()
    }

    fn validate_opcode(&self) -> Result<(), Error> {
        if self.opcode > 15 {
            Err(Error::InvalidOpcode)
        } else {
            Ok(())
        }
    }

    fn validate_reg_a(&self) -> Result<(), Error> {
        if self.input_a >= NUM_REGISTERS as u64 {
            Err(Error::InvalidInputA)
        } else {
            Ok(())
        }
    }

    fn validate_reg_b(&self) -> Result<(), Error> {
        if self.input_b >= NUM_REGISTERS as u64 {
            Err(Error::InvalidInputB)
        } else {
            Ok(())
        }
    }

    fn validate_reg_c(&self) -> Result<(), Error> {
        if self.output_c >= NUM_REGISTERS as u64 {
            Err(Error::InvalidOutputC)
        } else {
            Ok(())
        }
    }

    fn dispatch(&self, input: State) -> Result<State, Error> {
        self.validate_opcode()?;
        match self.opcode {
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
    ip: u64,
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

    fn run_program(&mut self) -> u64 {
        let mut reg_d_set: HashSet<u64> = HashSet::new();
        let mut prev_d = 0;
        while (self.ip as usize) < self.program.len() {
            // Write the IP to its bound register
            self.state[self.ip_reg] = self.ip;

            // For this script, this line potentially exits
            if self.ip == 28 {
                if reg_d_set.insert(self.state[3u64]) == false {
                    return prev_d;
                }
                prev_d = self.state[3u64];
            }

            // Execute the instruction
            self.state = self.program[self.ip as usize]
                .dispatch(self.state.clone())
                .unwrap();

            // Write the register value back to the IP (plus an increment)
            self.ip = self.state[self.ip_reg] + 1;
        }

        self.state[0u64]
    }
}

#[aoc(day21, part2)]
pub fn solve(input: &str) -> u64 {
    let mut chronal = ChronalComputer::from_string(input);
    let result = chronal.run_program();
    println!("Last halt: {}", result);
    assert_eq!(result, 7877093);
    result
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)] // No tests for this module
    use super::*;

    #[test]
    fn test_() {}
}

/*
#ip 2
00  seti 123 0 3        D = 123
01  bani 3 456 3        D &= 456
02  eqri 3 72 3         D = (D == 72)
03  addr 3 2 2          ip += D             Jump to line 5
04  seti 0 0 2          ip = 0              Jump to line 1
05  seti 0 6 3          D = 0
06  bori 3 65536 4      E = D | 65536
07  seti 7041048 8 3    D = 7041048
08  bani 4 255 5        F = E & 255
09  addr 3 5 3          D += F
10  bani 3 16777215 3   D &= 16777215
11  muli 3 65899 3      D *= 65899
12  bani 3 16777215 3   D &= 16777215
13  gtir 256 4 5        F = (256 > E)
14  addr 5 2 2          ip += F             Jump to line 15 or 16
15  addi 2 1 2          ip += 1             Jump to line 17
16  seti 27 6 2         ip = 27             Jump to line 28
17  seti 0 1 5          F = 0
18  addi 5 1 1          B = F + 1
19  muli 1 256 1        B *= 256
20  gtrr 1 4 1          B = (B > E)
21  addr 1 2 2          ip += B             Jump to line 22 or 23
22  addi 2 1 2          ip += 1             Jump to line 24
23  seti 25 1 2         ip = 25             Jump to line 26
24  addi 5 1 5          F += 1
25  seti 17 8 2         ip = 17             Jump to line 18
26  setr 5 2 4          E = F
27  seti 7 9 2          ip = 7              Jump to line 8
28  eqrr 3 0 5          F = (D == A)
29  addr 5 2 2          ip += F             Jump to line 30 or exit
30  seti 5 3 2          ip = 5              Jump to line 6
*/