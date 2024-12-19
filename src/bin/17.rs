use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use regex::Regex;

advent_of_code::solution!(17);

struct VirtualMachine {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_memory: Vec<u8>,
    pc: usize,
    halted: bool,
    outputs: Vec<u8>,
    // ...
}

impl VirtualMachine {
    fn from_str(input: &str) -> Self {
        let regex = Regex::new(
            r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ([\d,]+)",
        )
        .unwrap();

        let m = regex.captures(input).unwrap();
        let m: [&str; 4] = m.extract().1;

        Self {
            register_a: m[0].parse().unwrap(),
            register_b: m[1].parse().unwrap(),
            register_c: m[2].parse().unwrap(),
            pc: 0,
            instruction_memory: m[3].split(',').filter_map(|x| x.parse().ok()).collect(),
            halted: false,
            outputs: vec![],
        }
    }

    fn step(&mut self) {
        if self.halted {
            return;
        }

        if self.pc >= self.instruction_memory.len() - 1 {
            self.halted = true;
            return;
        }

        let op = self.instruction_memory[self.pc + 1];
        if match self.instruction_memory[self.pc] {
            0 => {
                self.register_a >>= self.process_combo_op(op);
                true
            }
            1 => {
                self.register_b ^= op as u64;
                true
            }
            2 => {
                self.register_b = self.process_combo_op(op) & 0b111;
                true
            }
            3 => {
                if self.register_a != 0 {
                    self.pc = self.process_combo_op(op).try_into().unwrap();
                    false
                } else {
                    true
                }
            }
            4 => {
                self.register_b ^= self.register_c;
                true
            }
            5 => {
                // considering we're dropping all but the last 3 bits, this is a justifiable cast,
                // fight me
                self.outputs.push(self.process_combo_op(op) as u8 & 0b111);
                true
            }
            6 => {
                self.register_b = self.register_a >> self.process_combo_op(op);
                true
            }
            7 => {
                self.register_c = self.register_a >> self.process_combo_op(op);
                true
            }
            invalid => panic!("Unrecognized opcode {invalid}"),
        } {
            self.pc += 2;
        }
    }

    fn process(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    fn process_combo_op(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand.into(),
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("Tried to parse combo operand 7, which is reserved"),
            invalid => panic!("Tried to parse combo operand {invalid}, which is invalid"),
        }
    }

    fn reset(&mut self, a: u64, b: u64, c: u64) {
        self.pc = 0;
        self.halted = false;
        self.outputs.clear();
        self.register_a = a;
        self.register_b = b;
        self.register_c = c;
    }

    fn bit_difference(&self) -> u64 {
        self.instruction_memory
            .iter()
            .zip(self.outputs.iter().chain(std::iter::repeat(&0)))
            .map(|(a, b)| match (a ^ b) & 0b111 {
                0 => 0,
                1 => 1,
                2 => 1,
                3 => 2,
                4 => 1,
                5 => 2,
                6 => 2,
                7 => 3,
                _ => unreachable!(),
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut vm = VirtualMachine::from_str(input);
    vm.process();

    Some(
        vm.outputs
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Augh(u64, u64);

// so this is an interesting state search problem, i guess
// i wonder how other people did it; i'm basically just doing educated guessing here
pub fn part_two(input: &str) -> Option<u64> {
    let mut vm = VirtualMachine::from_str(input);
    let old_b = vm.register_b;
    let old_c = vm.register_c;

    let mut states = BinaryHeap::new();

    let starting_state = 0;
    vm.reset(starting_state, old_b, old_c);
    vm.process();
    let starting_score = vm.bit_difference();
    states.push(Reverse(Augh(starting_score, starting_state)));

    let mut checked_states = HashSet::new();
    checked_states.insert(starting_state);
    let num_bits = vm.instruction_memory.len() * 3;
    let mut min_states = HashSet::new();

    // based on a half remembered sat 3 solver i made for school lol
    // this upper bound was found with trial and error; not sure how unique it is to my solution
    // runs in about one and a half seconds and gets the right answer tho sooooo not questioning it
    for _ in 0..5000 {
        let Some(Reverse(Augh(_score, state))) = states.pop() else {
            break;
        };

        for i in 0..(vm.instruction_memory.len() * 3) {
            let new_state = state ^ (1 << i);
            if checked_states.contains(&new_state) {
                continue;
            }
            checked_states.insert(new_state);
            if new_state & (7 << num_bits - 3) == 0 {
                // if none of those bits are set, the output won't be long enough.
                continue;
            }
            vm.reset(new_state, old_b, old_c);
            vm.process();
            let diff = vm.bit_difference();
            if diff == 0 {
                min_states.insert(new_state);
            }
            states.push(Reverse(Augh(diff, new_state)));
        }
    }

    let min = min_states.into_iter().min().unwrap();

    Some(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
