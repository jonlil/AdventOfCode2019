use std::io::{self, BufRead};

struct Program {
    data: Vec<i32>,
    index: usize,
}

impl Program {
    fn value(&self, index: usize) -> i32 {
        self.data[index]
    }

    fn write(&mut self, index: usize, input: i32) {
        self.data[index] = input;
    }

    fn get_parameter(&mut self, parameter: &Parameter, index: usize) -> i32 {
        match parameter.mode {
            ParameterMode::Position => self.value(self.value(index) as usize),
            ParameterMode::Immediate => self.value(index),
        }
    }

    fn addition(&mut self, parameters: Vec<Parameter>) {
        let first_part = self.get_parameter(&parameters[0], self.index + 0);
        let second_part = self.get_parameter(&parameters[1], self.index + 1);
        let destination = self.value(self.index + 2);

        self.write(destination as usize, first_part + second_part);
    }

    fn multiply(&mut self, parameters: Vec<Parameter>) {
        let first_part = self.get_parameter(&parameters[0], self.index + 0);
        let second_part = self.get_parameter(&parameters[1], self.index + 1);
        let destination = self.value(self.index + 2);

        self.write(destination as usize, first_part * second_part);
    }

    fn jump_if_true(&mut self, parameters: Vec<Parameter>) {
        if self.get_parameter(&parameters[0], self.index + 1) != 0 {
            self.index = self.get_parameter(&parameters[1], self.index + 2) as usize;
        } else {
            self.index += 3;
        }
    }

    fn jump_if_false(&mut self, parameters: Vec<Parameter>) {
        if self.get_parameter(&parameters[0], self.index + 1) == 0 {
            self.index = self.get_parameter(&parameters[1], self.index + 2) as usize;
        } else {
            self.index += 3;
        }
    }

    fn less_then(&mut self, parameters: Vec<Parameter>) {
        let first = self.get_parameter(&parameters[0], self.index + 1);
        let second = self.get_parameter(&parameters[1], self.index + 2);
        let destination = self.value(self.index + 3);

        let output = if first < second {
            1
        } else {
            0
        };

        self.write(destination as usize, output);
        self.index += 4;
    }

    fn equals(&mut self, parameters: Vec<Parameter>) {
        let first = self.get_parameter(&parameters[0], self.index + 1);
        let second = self.get_parameter(&parameters[1], self.index + 2);
        let destination = self.value(self.index + 3);

        let output = if first == second {
            1
        } else {
            0
        };

        self.write(destination as usize, output);
        self.index += 4;
    }
}

impl From<Vec<i32>> for Program {
    fn from(data: Vec<i32>) -> Program {
        Program {
            data: data,
            index: 0,
        }
    }
}

impl std::ops::Index<usize> for Program {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut program: Program = Program::from(io::stdin().lock().lines()
        .map(|line| parse_program_into_instructions(&line.unwrap()))
        .flatten()
        .collect::<Vec<i32>>()
    );

    let input = 5;

    loop {
        let instruction = Instruction::parse(program[program.index]).unwrap();
        match instruction.opcode {
            Opcode::Input => {
                program.index += 1;
                program.write(program.value(program.index) as usize, input);
                program.index += 1;
            },
            Opcode::Addition => {
                program.index += 1;
                program.addition(instruction.parameters);
                program.index += 3;
            },
            Opcode::Multiplication => {
                program.index += 1;
                program.multiply(instruction.parameters);
                program.index += 3;
            },
            Opcode::JumpIfTrue => {
                program.jump_if_true(instruction.parameters);
            },
            Opcode::JumpIfFalse => {
                program.jump_if_false(instruction.parameters);
            },
            Opcode::LessThen => {
                program.less_then(instruction.parameters);
            },
            Opcode::Equals => {
                program.equals(instruction.parameters);
            },
            Opcode::Output => {
                program.index += 1;
                eprintln!("{}", program.value(program.value(program.index) as usize));
                program.index += 1;
            },
            Opcode::Exit => {
                break;
            },
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<&u32> for ParameterMode {
    fn from(input: &u32) -> ParameterMode {
        if input == &0 {
            ParameterMode::Position
        } else if input == &1 {
            ParameterMode::Immediate
        } else {
            panic!("Unsupported parameter mode")
        }
    }
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Addition,
    Multiplication,
    JumpIfTrue,
    JumpIfFalse,
    LessThen,
    Equals,
    Input,
    Output,
    Exit,
}

impl From<u32> for Opcode {
    fn from(number: u32) -> Opcode {
        match number {
            1 => Self::Addition,
            2 => Self::Multiplication,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThen,
            8 => Self::Equals,
            99 => Self::Exit,
            opcode => panic!("Unknown opcode: {}", opcode),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: Opcode,
    parameters: Vec<Parameter>,
}

#[derive(Debug, PartialEq)]
struct Parameter {
    mode: ParameterMode,
    position: usize,
}

impl Instruction {
    fn normalize(input: i32) -> Result<[u32; 4], &'static str> {
        if input > 99999 {
            return Err("overflow opcode")
        }

        let mut iterator: Vec<u32> = input.to_string()
            .chars()
            .map(|s| s.to_digit(10).unwrap())
            .collect();
        iterator.reverse();

        let mut parameters = [0u32; 5];
        let mut index = 5;

        for value in iterator {
            parameters[index - 1] = value;
            index -= 1;
        }

        parameters[3] = format!("{}{}", parameters[3], parameters[4]).parse::<u32>().unwrap();

        Ok([
            parameters[0],
            parameters[1],
            parameters[2],
            parameters[3],
        ])
    }

    fn parse(input: i32) -> Result<Instruction, &'static str> {
        let parameters = Self::normalize(input)?;
        let opcode = Opcode::from(parameters[3]);

        let parameters = match &opcode {
            Opcode::Exit => vec![],
            Opcode::Input | Opcode::Output => {
                vec![Parameter {
                    mode: ParameterMode::Position,
                    position: 0,
                }]
            },
            _ => {
                parameters[0..=2].into_iter()
                    .rev()
                    .map(ParameterMode::from)
                    .enumerate()
                    .map(|(position, mode)| Parameter { mode, position })
                    .collect::<Vec<Parameter>>()
            },
        };

        Ok(Instruction {
            opcode,
            parameters,
        })
    }
}

fn parse_program_into_instructions(raw: &str) -> Vec<i32> {
    raw.split(',').map(|s| s.parse::<i32>().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_decodes_raw_program_instructions() {
        let raw_line: &str = "1002,4,3,4,33";
        let instructions = parse_program_into_instructions(&raw_line);

        assert_eq!(instructions, vec![1002, 4, 3, 4, 33]);
    }

    #[test]
    fn it_can_normalize_instruction() {
        assert_eq!(Instruction::normalize(111112), Err("overflow opcode"));
        assert_eq!(Instruction::normalize(11112), Ok([1, 1, 1, 12]));
        assert_eq!(Instruction::normalize(1003), Ok([0, 1, 0, 3]));
        assert_eq!(Instruction::normalize(99), Ok([0, 0, 0, 99]));
        assert_eq!(Instruction::normalize(1003), Ok([0, 1, 0, 3]));

    }

    #[test]
    fn it_parses_opcode_instructions() {
        assert_eq!(Instruction::parse(2), Ok(Instruction {
            opcode: Opcode::Multiplication,
            parameters: vec![
                Parameter {
                    mode: ParameterMode::Position,
                    position: 0,
                },
                Parameter {
                    mode: ParameterMode::Position,
                    position: 1,
                },
                Parameter {
                    mode: ParameterMode::Position,
                    position: 2,
                },
            ],
        }));

        assert_eq!(Instruction::parse(0102), Ok(Instruction {
            opcode: Opcode::Multiplication,
            parameters: vec![
                Parameter {
                    mode: ParameterMode::Immediate,
                    position: 0,
                },
                Parameter {
                    mode: ParameterMode::Position,
                    position: 1,
                },
                Parameter {
                    mode: ParameterMode::Position,
                    position: 2,
                },
            ],
        }));

        assert_eq!(Instruction::parse(1002), Ok(Instruction {
            opcode: Opcode::Multiplication,
            parameters: vec![
                Parameter {
                    mode: ParameterMode::Position,
                    position: 0,
                },
                Parameter {
                    mode: ParameterMode::Immediate,
                    position: 1,
                },
                Parameter {
                    mode: ParameterMode::Position,
                    position: 2,
                },
            ],
        }));

        assert_eq!(Instruction::parse(3), Ok(Instruction {
            opcode: Opcode::Input,
            parameters: vec![
                Parameter {
                    mode: ParameterMode::Position,
                    position: 0,
                },
            ],
        }));
    }
}
