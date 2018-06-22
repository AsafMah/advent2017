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
    Snd(Value),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Rcv(Value),
    Jgz(Value, Value),
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
        CompleteStr("snd") => map!(value, |n| Instruction::Snd(n)) |
        CompleteStr("set") => map!(separated_pair!(register, space, value), |(v1, v2)| Instruction::Set(v1, v2)) |
        CompleteStr("add") => map!(separated_pair!(register, space, value), |(v1, v2)| Instruction::Add(v1, v2)) |
        CompleteStr("mul") => map!(separated_pair!(register, space, value), |(v1, v2)| Instruction::Mul(v1, v2)) |
        CompleteStr("mod") => map!(separated_pair!(register, space, value), |(v1, v2)| Instruction::Mod(v1, v2)) |
        CompleteStr("rcv") => map!(value, |n| Instruction::Rcv(n)) |
        CompleteStr("jgz") => map!(separated_pair!(value, space, value), |(v1, v2)| Instruction::Jgz(v1, v2))
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

    fn do_instruction(&mut self) -> Option<Option<i64>> {
        if let Some(&ins) = self.instructions.get(self.ip) {
            self.ip += 1;
            match ins {
                Instruction::Snd(v) => {
                    self.memory = self.get_value(v) }
                Instruction::Set(Register(r), v) => { self.registers[r] = self.get_value(v) }
                Instruction::Add(Register(r), v) => { self.registers[r] += self.get_value(v) }
                Instruction::Mul(Register(r), v) => { self.registers[r] *= self.get_value(v) }
                Instruction::Mod(Register(r), v) => { self.registers[r] %= self.get_value(v) }
                Instruction::Rcv(v) => {
                    if self.get_value(v) != 0 {
                        return Some(Some(self.memory));
                    }
                    }
                Instruction::Jgz(v, o) => {
                    if self.get_value(v) > 0 {
                        self.ip -= 1; //reset to before increment
                        self.ip = ((self.ip as i64) + self.get_value(o)) as usize;
                    }
                }
            }
            Some(None)
        } else {
            None
        }
    }
}

impl Iterator for State {
    type Item = Option<i64>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.do_instruction()
    }
}

fn main() -> Result<(), Box<Error>> {
    let input: &'static str = include_str!("input_day_18");
    let input = CompleteStr(input);
    let ins = instructions(input);
    eprintln!("ins = {:?}", ins);
    let state = State::new(ins.unwrap().1);

    for i in state {
        if let Some(i) = i {
            eprintln!("rcv value = {:?}", i);
            break;
        }

    }


    Ok(())
}