/*
    --- Day 21: Chronal Conversion ---
    You should have been watching where you were going, because as you wander the new North Pole base, you trip and fall into a very deep hole!

    Just kidding. You're falling through time again.

    If you keep up your current pace, you should have resolved all of the temporal anomalies by the next time the device activates. Since you have very little interest in browsing history in 500-year increments for the rest of your life, you need to find a way to get back to your present time.

    After a little research, you discover two important facts about the behavior of the device:

    First, you discover that the device is hard-wired to always send you back in time in 500-year increments. Changing this is probably not feasible.

    Second, you discover the activation system (your puzzle input) for the time travel module. Currently, it appears to run forever without halting.

    If you can cause the activation system to halt at a specific moment, maybe you can make the device send you so far back in time that you cause an integer underflow in time itself and wrap around back to your current time!

    The device executes the program as specified in manual section one and manual section two.

    Your goal is to figure out how the program works and cause it to halt. You can only control register 0; every other register begins at 0 as usual.

    Because time travel is a dangerous activity, the activation system begins with a few instructions which verify that bitwise AND (via bani) does a numeric operation and not an operation as if the inputs were interpreted as strings. If the test fails, it enters an infinite loop re-running the test instead of allowing the program to execute normally. If the test passes, the program continues, and assumes that all other bitwise operations (banr, bori, and borr) also interpret their inputs as numbers. (Clearly, the Elves who wrote this system were worried that someone might introduce a bug while trying to emulate this system with a scripting language.)

    What is the lowest non-negative integer value for register 0 that causes the program to halt after executing the fewest instructions? (Executing the same instruction multiple times counts as multiple instructions executed.)
*/

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, space1},
    combinator::map_res,
    multi::many1,
    sequence::{pair, preceded, tuple},
    IResult,
};
use std::ops::{Index, IndexMut};

const NUM_REGISTERS: usize = 6;

#[derive(Clone, Debug, Eq, PartialEq)]
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
    Opcode,
    Functioncode,
    InputA,
    InputB,
    OutputC,
}

struct Instruction {
    opcode: u8,
    input_a: u64,
    input_b: u64,
    output_c: u64,
}

impl Instruction {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (name, a, b, c)) = tuple((
            preceded(multispace0, alpha1),
            preceded(space1, map_res(digit1, |a: &str| a.parse::<u64>())),
            preceded(space1, map_res(digit1, |b: &str| b.parse::<u64>())),
            preceded(space1, map_res(digit1, |c: &str| c.parse::<u64>())),
        ))(input)?;

        let opcode = match name {
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
            _ => panic!("Invalid opcode"),
        };

        Ok((
            input,
            Self {
                opcode,
                input_a: a,
                input_b: b,
                output_c: c,
            },
        ))
    }

    fn validate_opcode(&self) -> Result<(), Error> {
        if self.opcode > 15 {
            Err(Error::Opcode)
        } else {
            Ok(())
        }
    }

    fn validate_reg_a(&self) -> Result<(), Error> {
        if self.input_a >= NUM_REGISTERS as u64 {
            Err(Error::InputA)
        } else {
            Ok(())
        }
    }

    fn validate_reg_b(&self) -> Result<(), Error> {
        if self.input_b >= NUM_REGISTERS as u64 {
            Err(Error::InputB)
        } else {
            Ok(())
        }
    }

    fn validate_reg_c(&self) -> Result<(), Error> {
        if self.output_c >= NUM_REGISTERS as u64 {
            Err(Error::OutputC)
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
            _ => Err(Error::Functioncode),
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
        Self::parser(input).unwrap().1
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (ip_reg, program)) = pair(
            preceded(tag("#ip "), map_res(digit1, |x: &str| x.parse::<usize>())),
            many1(Instruction::parser),
        )(input)?;

        Ok((
            input,
            Self {
                state: State([0; NUM_REGISTERS]),
                program,
                ip_reg,
                ip: 0,
            },
        ))
    }

    fn run_program(&mut self) -> u64 {
        while (self.ip as usize) < self.program.len() {
            // Write the IP to its bound register
            self.state[self.ip_reg] = self.ip;

            // For this script, this line potentially exits
            if self.ip == 28 {
                return self.state[3u64]; // Register 'D'
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

#[aoc(day21, part1)]
pub fn solve(input: &str) -> u64 {
    let mut chronal = ChronalComputer::from_string(input);
    let result = chronal.run_program();
    println!("First halt: {}", result);
    assert_eq!(result, 9107763);
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
