#[macro_use]
extern crate nom;

use std::error::Error;
use nom::{alpha, space, line_ending};
use nom::types::CompleteStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Register(usize);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Value {
    Register(Register),
    Number(i64),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Instruction {
    Set(Register, Value),
    Add(Register, Value),
    Sub(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Jnz(Value, Value),
}

fn is_number(chr: char) -> bool {
    chr >= '0' && chr <= '9' || chr == '-'
}

named!(register<CompleteStr, Register>,
    map!(take!(1), |b| Register((b.as_bytes()[0] as usize) - ('a' as usize)) )
);

named!(number<CompleteStr, i64>,
    flat_map!(take_while!(is_number), parse_to!(i64))
);

named!(value<CompleteStr, Value>,
alt!(
    number => {|n| Value::Number(n)} |
    register => {|r| Value::Register(r) }
)
);

named!(instruction<CompleteStr, Instruction>,
    switch!(ws!(alpha),
        CompleteStr("set") => map!(separated_pair!(register, space, value), |(v1, v2)| Instruction::Set(v1, v2)) |
        CompleteStr("add") => map!(separated_pair!(register, space, value), |(v1, v2)| Instruction::Add(v1, v2)) |
        CompleteStr("sub") => map!(separated_pair!(register, space, value), |(v1, v2)| Instruction::Sub(v1, v2)) |
        CompleteStr("mul") => map!(separated_pair!(register, space, value), |(v1, v2)| Instruction::Mul(v1, v2)) |
        CompleteStr("mod") => map!(separated_pair!(register, space, value), |(v1, v2)| Instruction::Mod(v1, v2)) |
        CompleteStr("jnz") => map!(separated_pair!(value, space, value), |(v1, v2)| Instruction::Jnz(v1, v2))
    )
);

named!(instructions<CompleteStr, Vec<Instruction>>,
    separated_list!(line_ending , instruction)
);


#[derive(Debug)]
struct State {
    registers: Vec<i64>,
    ip: usize,
    instructions: Vec<Instruction>,
    memory: i64,
}

impl State {
    fn new(instructions: Vec<Instruction>) -> State {
        State {
            memory: 0,
            ip: 0,
            registers: (0..256).map(|_| 0).collect(),
            instructions,
        }
    }

    fn get_value(&self, v: Value) -> i64 {
        match v {
            Value::Register(Register(r)) => self.registers[r],
            Value::Number(i) => i,
        }
    }

    fn do_instruction(&mut self) -> Option<bool> {
        let mut did_mul = false;
        if let Some(&ins) = self.instructions.get(self.ip) {
            self.ip += 1;
            match ins {
                Instruction::Set(Register(r), v) => { self.registers[r] = self.get_value(v) }
                Instruction::Add(Register(r), v) => { self.registers[r] += self.get_value(v) }
                Instruction::Sub(Register(r), v) => { self.registers[r] -= self.get_value(v) }
                Instruction::Mul(Register(r), v) => { did_mul = true;self.registers[r] *= self.get_value(v) }
                Instruction::Mod(Register(r), v) => { self.registers[r] %= self.get_value(v) }
                Instruction::Jnz(v, o) => {
                    if self.get_value(v) != 0 {
                        self.ip -= 1; //reset to before increment
                        self.ip = ((self.ip as i64) + self.get_value(o)) as usize;
                    }
                }
            }
            Some(did_mul)
        } else {
            None
        }
    }
}

impl Iterator for State {
    type Item = bool;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.do_instruction()
    }
}

fn main() -> Result<(), Box<Error>> {
    let input: &'static str = include_str!("input_day_23");
    let input = CompleteStr(input);
    let ins = instructions(input);
    let state = State::new(ins.unwrap().1);
    eprintln!("state = {:?}", state);
    let mut mul_counter = 0;

    for did_mul in state {
        if did_mul {
            mul_counter += 1;
        }

    }

    eprintln!("mul_counter = {:?}", mul_counter);

    Ok(())
}