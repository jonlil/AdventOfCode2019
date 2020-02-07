use std::io::{self, BufRead};

struct NounVerb {
    stop: usize,
    curr: (u32, u32),
    next: (u32, u32),
}

impl NounVerb {
    pub fn new(stop: usize) -> Self {
        Self {
            stop: stop,
            curr: (0, 0),
            next: (0, 0),
        }
    }
}

impl Iterator for NounVerb {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.curr = self.next;

        if self.next.0 > self.stop as u32 {
            return None;
        }

        if self.next.1 > 0 && self.next.1 == self.stop as u32 {
            self.next.0 += 1;
            self.next.1 = 0;
        } else {
            self.next.1 += 1;
        }

        Some(self.curr)
    }
}

fn main() {
    let stdin = io::stdin();

    let mut source_opcodes: Vec<u32> = vec![];
    for line in stdin.lock().lines() {
        source_opcodes.extend(parse_opcode_into_vector(&line.unwrap()));
    }

    //for (noun, verb) in NounVerb::new(99) {
    //    let mut opcodes = source_opcodes.clone();
    //    opcodes[1] = noun;
    //    opcodes[2] = verb;

    //    let result = process(&mut opcodes)[0];
    //    if result == 19690720_u32 {
    //        eprintln!("Noun: {} with Verb: {} produces correct result: 100 * noun * verb is: {}", noun, verb, (100 * noun + verb));
    //        break;
    //    } else {
    //        eprintln!("Combo {}, {} produced result: {}", noun, verb, result);
    //    }
    //}
}

enum OptcodeInstruction {
    Addition,
    Multiplication,
    Exit,
}

impl OptcodeInstruction {
    fn decode(input: &u32) -> Self {
        match input {
            1 => Self::Addition,
            2 => Self::Multiplication,
            99 => Self::Exit,
            _ => panic!("Unknown opcode"),
        }
    }
}

fn parse_opcode_into_vector(raw: &str) -> Vec<u32> {
  raw.split(',').map(|s| s.parse::<u32>().unwrap()).collect()
}

enum ParameterMode {
    Position,
    Immediate,
}

fn get_parameter_positions(
    a: usize,
    b: usize,
    c: usize,
    items: &Vec<u32>
) -> ((usize, usize), usize) {
    ((items[a] as usize, items[b] as usize), items[c] as usize)
}

fn process(opcodes: &mut Vec<u32>) -> Vec<u32> {
    let mut index: usize = 0;
    let total_indexes = opcodes.len() - 1;

    loop {
        let instruction_pointer = OptcodeInstruction::decode(&opcodes[index]);
        match instruction_pointer {
            OptcodeInstruction::Exit => break,
            _ => {},
        }

        let (parameters, address) = get_parameter_positions(
            index + 1,
            index + 2,
            index + 3,
            &opcodes,
        );

        match instruction_pointer {
            OptcodeInstruction::Addition => {
                opcodes[address] = opcodes[parameters.0] + opcodes[parameters.1];
                index += 4;
            },
            OptcodeInstruction::Multiplication => {
                opcodes[address] = opcodes[parameters.0] * opcodes[parameters.1];
                index += 4;
            },
            OptcodeInstruction::Exit => {
                index += 1;
            },
        };

        if index > total_indexes {
            break;
        }
    }

    opcodes.to_vec()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_parse_opcode_into_vector_of_codes() {
        let line: &str = "1,9,10,3,2,3,11,0,99,30,40,50";
        assert_eq!(parse_opcode_into_vector(&line).len(), 12);
    }

    #[test]
    fn it_decodes_optcodes_into_instructions() {
        let raw_line: &str = "1002,4,3,4,33";
        let opcodes = parse_opcode_into_vector(&raw_line);
    }
}
