use crate::value::Value;
use anyhow::{bail, Error, Result};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum OpCode {
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

impl TryFrom<u8> for OpCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match num_traits::FromPrimitive::from_u8(value) {
            Some(v) => Ok(v),
            None => bail!("Failed to convert"),
        }
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    lines: Vec<u32>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: vec![],
            lines: vec![],
            constants: vec![],
        }
    }

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        <usize as TryInto<u8>>::try_into(self.constants.len()).unwrap() - 1
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:4} ");
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }
        let instruction = self.code[offset];
        let op_code: Result<OpCode> = instruction.try_into();
        match op_code {
            Ok(OpCode::Constant) => self.constant_instruction("Constant", offset),
            Ok(OpCode::Add) => self.simple_instruction("Add", offset),
            Ok(OpCode::Subtract) => self.simple_instruction("Subtract", offset),
            Ok(OpCode::Multiply) => self.simple_instruction("Multiply", offset),
            Ok(OpCode::Divide) => self.simple_instruction("Divide", offset),
            Ok(OpCode::Negate) => self.simple_instruction("Negate", offset),
            Ok(OpCode::Return) => self.simple_instruction("Return", offset),
            Err(_) => {
                println!("Unknown opcode {instruction}");
                offset + 1
            }
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let index = self.code[offset + 1];
        println!("{name} {:4} '{}'", index, self.constants[index as usize]);
        offset + 2
    }
}
