/*
    --- Part Two ---
    Using the samples you collected, work out the number of each opcode and execute the test program (the second section of your puzzle input).

    What value is contained in register 0 after executing the test program?
*/

use regex::Regex;
use std::ops::{ Index, IndexMut };

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
        let mut split = input.trim().split(" ");

        Self {
            opcode:   split.next().unwrap().parse::<u8>().unwrap(),
            input_a:  split.next().unwrap().parse::<u32>().unwrap(),
            input_b:  split.next().unwrap().parse::<u32>().unwrap(),
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
    fn many_from_string(input: &str) -> (Vec<Self>, usize) { // A vector of samples, and the offset into the input where matching finished consuming
        let re = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]\n((?:\d+\s*)+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
        let mut consumed_offset = 0;
        let samples = re.captures_iter(input)
            .map(|cap| {
                consumed_offset = cap.get(cap.len() - 1).unwrap().end(); // The last capture's endpoint in the haystack
                Self {
                    before: State([cap[1].parse::<u32>().unwrap(),
                                   cap[2].parse::<u32>().unwrap(),
                                   cap[3].parse::<u32>().unwrap(),
                                   cap[4].parse::<u32>().unwrap()]),
                    op: Instruction::from_string(&cap[5]),
                    after: State([cap[6].parse::<u32>().unwrap(),
                                  cap[7].parse::<u32>().unwrap(),
                                  cap[8].parse::<u32>().unwrap(),
                                  cap[9].parse::<u32>().unwrap()]),
                }
            })
            .collect();
        (samples, consumed_offset)
    }

    fn find_possible_opcodes(&self) -> Vec<u8> { // Returns a vec of all possible function codes
        (0..16).map(|i| (i, self.op.dispatch_as(self.before.clone(), i)))
            .filter(|(_i, result)|
                match result {
                    Ok(x) => x == &self.after,
                    _ => false,
                })
            .map(|(i, _result)| i)
            .collect()
    }
}

struct ChronalComputer {
    state: State,
    program: Vec<Instruction>,
    opcode_lookup: [u8; 16]
}

impl ChronalComputer {
    fn from_string(input: &str) -> Self {
        let (samples, offset) = Sample::many_from_string(input);
        let opcode_lookup = ChronalComputer::deduce_opcodes(&samples);

        let re = Regex::new(r"\d+ \d+ \d+ \d+").unwrap();
        let program: Vec<Instruction> = re.captures_iter(&input[offset..])
            .map(|cap| Instruction::from_string(&cap[0]))
            .collect();

        Self {
            state: State([0, 0, 0, 0]),
            program: program,
            opcode_lookup: opcode_lookup,
        }
    }

    fn deduce_opcodes(samples: &Vec<Sample>) -> [u8; 16] {
        let mut decoded: [Option<u8>; 16] = [None; 16]; // A mapping of opcode to function code

        let mut sample_func_codes: Vec<Vec<u8>> = samples.iter()
            .map(|sample| sample.find_possible_opcodes())
            .collect();

        while decoded.iter().filter(|op| op.is_none()).count() > 0 {
            for (sample, func_codes) in samples.iter().zip(sample_func_codes.iter_mut()) {
                // If the opcode is already known, verify that it is consistent with this sample's list of function codes
                // then clear the list. Otherwise, it's not known and we should remove any known opcodes from the list.
                // After that, if there is only one remaining function code in the list then it must be the one designated
                // for the opcode.
                if let Some(d) = decoded[sample.op.opcode as usize] {
                    // Opcode is known. Verify consistency.
                    if func_codes.len() > 0 {
                        assert!(func_codes.contains(&d) == true);
                    }
                    func_codes.clear();
                } else {
                    // Opcode currently unknown. Remove any function codes with known opcodes from this sample's list.
                    func_codes.retain(|&fc| decoded.contains(&Some(fc)) == false);

                    if func_codes.len() == 1 {
                        // There's only one possibility so we know what this opcode means now
                        decoded[sample.op.opcode as usize] = Some(func_codes[0]);
                        func_codes.clear();
                    }
                }
            }
        }

        // Convert from Option<u8> to u8
        let mut opcode_lookup: [u8; 16] = [0; 16];
        for i in 0..opcode_lookup.len() {
            opcode_lookup[i] = decoded[i].unwrap();
        }
        opcode_lookup
    }

    fn run_program(&mut self) -> u32 {
        self.state = self.program.iter()
            .fold(self.state.clone(), |state, instr| instr.dispatch(state, &self.opcode_lookup).unwrap());

        self.state[0]
    }
}

#[aoc(day16, part2)]
pub fn solve(input: &str) -> u32 {
    let mut computer = ChronalComputer::from_string(input);
    let output = computer.run_program();
    println!("Program output: {}", output);
    assert_eq!(output, 681);
    output
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
        let (samples, _offset) = Sample::many_from_string(input);
        let possible_opcodes = samples[0].find_possible_opcodes();
        assert_eq!(possible_opcodes, vec![1, 2, 9]); // addi, mulr, seti
    }
}
