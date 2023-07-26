use core::fmt;
use std::{array, mem};

use crate::value::Value;
use crate::{compiler, Chunk, OpCode};

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
    GreaterThan,
    LessThan,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> VM<'a> {
        VM {
            chunk,
            ip: 0,
            stack: array::from_fn(|_| Value::default()),
            stack_top: 0,
        }
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
                OpCode::Add => match self.binary_op(BinaryOp::Add) {
                    InterpretResult::CompileError => return InterpretResult::CompileError,
                    InterpretResult::RuntimeError => return InterpretResult::RuntimeError,
                    InterpretResult::Ok => (),
                },
                OpCode::Subtract => match self.binary_op(BinaryOp::Subtract) {
                    InterpretResult::CompileError => return InterpretResult::CompileError,
                    InterpretResult::RuntimeError => return InterpretResult::RuntimeError,
                    InterpretResult::Ok => (),
                },
                OpCode::Multiply => match self.binary_op(BinaryOp::Multiply) {
                    InterpretResult::CompileError => return InterpretResult::CompileError,
                    InterpretResult::RuntimeError => return InterpretResult::RuntimeError,
                    InterpretResult::Ok => (),
                },
                OpCode::Divide => match self.binary_op(BinaryOp::Divide) {
                    InterpretResult::CompileError => return InterpretResult::CompileError,
                    InterpretResult::RuntimeError => return InterpretResult::RuntimeError,
                    InterpretResult::Ok => (),
                },
                OpCode::Not => {
                    let v = self.pop();
                    self.push(Value::Bool(Self::is_falsey(v)));
                }
                OpCode::Negate => match self.peek(0) {
                    Value::Number(n) => {
                        self.pop();
                        self.push(Value::Number(-n));
                    }
                    _ => {
                        self.runtime_error(format_args!("Operand must be a number."));
                        return InterpretResult::RuntimeError;
                    }
                },
                OpCode::Constant => {
                    let constant = self.read_constant().clone();
                    self.push(constant);
                }
                OpCode::Nil => self.push(Value::Nil),
                OpCode::True => self.push(Value::Bool(true)),
                OpCode::False => self.push(Value::Bool(false)),
                OpCode::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Bool(a == b));
                }
                OpCode::Greater => match self.binary_op(BinaryOp::GreaterThan) {
                    InterpretResult::CompileError => return InterpretResult::CompileError,
                    InterpretResult::RuntimeError => return InterpretResult::RuntimeError,
                    InterpretResult::Ok => (),
                },
                OpCode::Less => match self.binary_op(BinaryOp::LessThan) {
                    InterpretResult::CompileError => return InterpretResult::CompileError,
                    InterpretResult::RuntimeError => return InterpretResult::RuntimeError,
                    InterpretResult::Ok => (),
                },
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
    fn read_constant(&mut self) -> &Value {
        let index = self.read_byte();
        &self.chunk.constants[index as usize]
    }

    #[inline(always)]
    fn binary_op(&mut self, op: BinaryOp) -> InterpretResult {
        match (self.peek(0), self.peek(1)) {
            (a @ _, b @ _) if a.is_string() && b.is_string() => {
                self.concatenate();
                InterpretResult::Ok
            }
            (Value::Number(a), Value::Number(b)) => {
                self.pop();
                self.pop();
                let c = match op {
                    BinaryOp::Add => Value::Number(a + b),
                    BinaryOp::Divide => Value::Number(a / b),
                    BinaryOp::Multiply => Value::Number(a * b),
                    BinaryOp::Subtract => Value::Number(a - b),
                    BinaryOp::GreaterThan => Value::Bool(a > b),
                    BinaryOp::LessThan => Value::Bool(a < b),
                };
                self.push(c);
                InterpretResult::Ok
            }
            _ => {
                self.runtime_error(format_args!("Operands must be two numbers or two strings."));
                InterpretResult::RuntimeError
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        let ret = mem::take(&mut self.stack[self.stack_top]);
        self.stack_top -= 1;
        ret
    }

    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack_top - 1 - distance].clone()
    }

    fn reset_stack(&mut self) {
        self.stack = array::from_fn(|_| Value::default());
        self.stack_top = 0;
    }

    fn runtime_error(&mut self, args: fmt::Arguments) {
        eprintln!("{args}");

        let instruction = self.ip - 1;
        let line = self.chunk.lines[instruction];
        eprintln!("[line {line}] in script");
        self.reset_stack();
    }

    fn is_falsey(value: Value) -> bool {
        match value {
            Value::Nil => true,
            Value::Bool(false) => true,
            _ => false,
        }
    }

    fn concatenate(&mut self) {
        let b_val = self.pop();
        let a_val = self.pop();
        let b = b_val.as_str().unwrap();
        let a = a_val.as_str().unwrap();
        let mut concatenated = String::from(a);
        concatenated.push_str(b);
        self.push(Value::from_string(concatenated));
    }
}

pub fn interpret(source: &str) -> InterpretResult {
    match compiler::compile(source) {
        Err(_) => return InterpretResult::CompileError,
        Ok(chunk) => {
            let mut vm = VM::new(&chunk);
            vm.run()
        }
    }
}
