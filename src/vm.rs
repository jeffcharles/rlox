use crate::value::Value;
use crate::{Chunk, OpCode};

const STACK_MAX: usize = 256;

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

#[must_use]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> VM<'a> {
        VM {
            chunk,
            ip: 0,
            stack: [Value::default(); STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn interpret(source: &str) -> InterpretResult {
        todo!();
        InterpretResult::Ok
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            if cfg!(feature = "debug_trace_execution") {
                print!("           ");
                for i in 0..self.stack_top {
                    print!("[ {} ]", self.stack[i]);
                }
                println!("");
                self.chunk.disassemble_instruction(self.ip);
            }
            let instruction = self.read_byte().try_into().unwrap();
            match instruction {
                OpCode::Return => {
                    let val = self.pop();
                    println!("{val}");
                    return InterpretResult::Ok;
                }
                OpCode::Add => self.binary_op(BinaryOp::Add),
                OpCode::Subtract => self.binary_op(BinaryOp::Subtract),
                OpCode::Multiply => self.binary_op(BinaryOp::Multiply),
                OpCode::Divide => self.binary_op(BinaryOp::Divide),
                OpCode::Negate => {
                    let val = self.pop();
                    self.push(-val);
                }
                OpCode::Constant => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
            }
        }
    }

    #[inline(always)]
    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    #[inline(always)]
    fn read_constant(&mut self) -> Value {
        let index = self.read_byte();
        self.chunk.constants[index as usize]
    }

    #[inline(always)]
    fn binary_op(&mut self, op: BinaryOp) {
        let b = self.pop();
        let a = self.pop();
        let c = match op {
            BinaryOp::Add => a + b,
            BinaryOp::Divide => a / b,
            BinaryOp::Multiply => a * b,
            BinaryOp::Subtract => a - b,
        };
        self.push(c);
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }
}
