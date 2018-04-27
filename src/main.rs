extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct State<'a>(HashMap<&'a str, i32>);

impl<'a> From<HashMap<&'a str, i32>> for State<'a> {
    fn from(map: HashMap<&'a str, i32>) -> Self {
        State(map)
    }
}

impl<'a> State<'a> {
    fn do_op(&mut self, op: &Op)
    {
        let if_val = self.0[op.if_reg];
        if !op.if_cond.is_correct(op.if_count, if_val) {
            return;
        }

        let val = match op.op_type {
            OpType::Inc => op.op_count,
            OpType::Dec => -op.op_count,
        };

        *self.0.get_mut(op.op_reg).unwrap() += val;
    }

    fn max(&self) -> i32 {
        self.0.iter().map(|(n,&i)| i).max().unwrap()
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Op<'a> {
    op_reg: &'a str,
    op_type: OpType,
    op_count: i32,
    if_reg: &'a str,
    if_cond: IfCond,
    if_count: i32,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum OpType {
    Inc,
    Dec,
}

impl FromStr for OpType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(OpType::Inc),
            "dec" => Ok(OpType::Dec),
            _ => Err(())
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum IfCond {
    Eq,
    Ne,
    Lt,
    Gt,
    Ge,
    Le,
}

impl IfCond {
    fn is_correct(&self, count: i32, val: i32) -> bool {
        match *self {
            IfCond::Eq => val == count,
            IfCond::Ne => val != count,
            IfCond::Lt => val < count,
            IfCond::Gt => val > count,
            IfCond::Ge => val >= count,
            IfCond::Le => val <= count,
        }
    }
}

impl FromStr for IfCond {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "==" => Ok(IfCond::Eq),
            "!=" => Ok(IfCond::Ne),
            "<" => Ok(IfCond::Lt),
            ">" => Ok(IfCond::Gt),
            "<=" => Ok(IfCond::Le),
            ">=" => Ok(IfCond::Ge),
            _ => Err(())
        }
    }
}

fn main() {
    let input: &'static str = include_str!("input_day_8");
    let regex = Regex::new(r"(?P<Reg>\w+) (?P<Type>inc|dec) (?P<Count>[-\d]+) if (?P<IfReg>\w+) (?P<IfCond>(==|>=|<=|>|<|!=)) (?P<IfCount>[-\d]+)").unwrap();

    let ops = input.lines().map(|line| {
        let cap = regex.captures(line).unwrap();
        Op {
            op_reg: cap.name("Reg").unwrap().as_str(),
            op_type: cap.name("Type").unwrap().as_str().parse().unwrap(),
            op_count: cap.name("Count").unwrap().as_str().parse().unwrap(),
            if_reg: cap.name("IfReg").unwrap().as_str(),
            if_cond: cap.name("IfCond").unwrap().as_str().parse().unwrap(),
            if_count: cap.name("IfCount").unwrap().as_str().parse().unwrap(),
        }
    }).collect::<Vec<_>>();

    let mut state = State(ops.iter()
        .map(|op| (op.op_reg, 0 as i32))
        .chain(
            ops.iter().map(|op| (op.if_reg, 0 as i32)))
        .collect());

    let mut max = 0;
    for op in ops {
        state.do_op(&op);

        max = match state.max() {
            m if m > max => m,
            _ => max
        };
    }

    eprintln!("max = {:?}", max);
}