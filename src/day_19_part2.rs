/*
    --- Part Two ---
    A new background process immediately spins up in its place. It appears identical, but on closer inspection, you notice that this time, register 0 started with the value 1.

    What value is left in register 0 when this new background process halts?
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
        let mut split = input.trim().split(" ");

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
            opcode:   opcode,
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
            program: program,
            ip_reg: ip_reg,
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

#[aoc(day19, part2)]
pub fn solve(_input: &str) -> u32 {
    // Running the given input for this problem would take until the heat death of the universe.
    // My solution is to modify the input program to make it more efficient.
    // Essentially, it is calculating divisors of a large number in the naieve way (test each combination of numbers which is O(n^n)).
    // The modified program only checks combinations below the sqrt of the large number. Annotations at the end of this file.
    let input = include_str!("day_19_part2_input.txt"); // Solution would take until the heat
    let mut chronal = ChronalComputer::from_string(input);
    chronal.state[0u32] = 1;
    let result = chronal.run_program();
    println!("Register 0: {}", result);
    assert_eq!(result, 13083798);
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

    #[test]
    fn test_equivalent_program() {
        let input = include_str!("day_19_part2_input.txt");
        let mut chronal = ChronalComputer::from_string(input);
        let result = chronal.run_program();
        assert_eq!(result, 1302);
    }
}

/*
#ip 5
addi 5 18 5     ip += 18
seti 1 3 1      B = 1
seti 1 1 2      C = 1
mulr 1 2 4      E = B * C
eqrr 4 3 4      E = (D == E)
addr 4 5 5      ip += E
addi 5 2 5      ip++
addr 1 0 0      A += B
setr 3 0 2      C = D
seti 35 35 5    ip = 35         // Jump to the new routine at the end of the program
addi 2 1 2      C++
gtrr 2 3 4      E = (C > D)
addr 5 4 5      ip += E
seti 2 4 5      ip = 2
addi 1 1 1      B++
gtrr 1 3 4      E = (B > D)
addr 4 5 5      ip += E
seti 1 5 5      ip = 1
mulr 5 5 5      ip *= ip
addi 3 76 3     D = 76          // Combine several lines to keep ip unchanged for subsequent lines
muli 3 11 3     D *= 11
addi 4 8 4      E += 8
mulr 4 5 4      E *= ip
addi 4 13 4     E += 13
addr 3 4 3      D += E
addr 5 0 5      ip += A
seti 0 8 5      ip = 0
setr 5 3 4      E = ip
mulr 4 5 4      E *= ip
addr 5 4 4      E += ip
mulr 5 4 4      E *= ip
muli 4 14 4     E *= 14
mulr 4 5 4      E *= ip
addr 3 4 3      D += E
seti 0 8 0      A = 0
seti 0 4 5      ip = 0
mulr 1 2 4      E = B * C
gtrr 3 4 4      E = (D > E)
addr 5 4 5      ip += E
setr 3 0 2      C = D
seti 9 9 5      ip = 9

equivalent to the following program:
    D = (2 * 2 * 19 * 11) + (8 * 22 + 13)
    if A == 1 {
        D += (27 * 28 + 29) * 30 * 14 * 32
        A = 0
    }

    for (B = 1; B <= D; B++)
        for (C = 1; C <= D; C++) {
            if (B * C == D) {
                A += B
                C = D // Force C loop to terminate
            }
            if (B * C >= D) {
                C = D // Force C loop to terminate
            }
        }
    }

    return A
*/