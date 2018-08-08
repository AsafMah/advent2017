extern crate failure;
extern crate itertools;
extern crate regex;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

#[derive(Copy, Debug, Clone)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            _ => Err(())
        }
    }
}

#[derive(Copy, Debug, Clone)]
struct Action {
    write_value: bool,
    direction: Direction,
    next_state: usize,
}

#[derive(Debug)]
struct State {
    if_0: Action,
    if_1: Action,
}

#[derive(Debug)]
struct Turing {
    current_state: usize,
    step_count: u32,
    states: Vec<State>,
    tape: Vec<bool>,
    cursor: usize,
}

fn letter_to_number(letter: &str) -> Result<usize, Error> {
    Ok(letter.chars().next().unwrap() as usize - 'A' as usize)
}

fn parse(input: &str) -> Result<Turing, Error> {
    let header_regex = Regex::new(r"Begin in state (?P<initial>\w+)\.
Perform a diagnostic checksum after (?P<steps>\d+) steps\.")?;

    let content_regex = Regex::new(r"In state (?P<state>\w+):
  If the current value is 0:
    - Write the value (?P<if_0_val>\d+)\.
    - Move one slot to the (?P<if_0_dir>\w+)\.
    - Continue with state (?P<if_0_state>\w+)\.
  If the current value is 1:
    - Write the value (?P<if_1_val>\d+)\.
    - Move one slot to the (?P<if_1_dir>\w+)\.
    - Continue with state (?P<if_1_state>\w+)\.")?;

    let captures = header_regex.captures(input).unwrap();
    let initial = letter_to_number(&captures["initial"])?;
    let step_count: u32 = captures["steps"].parse()?;

    let rest_of_input = &input[captures.get(0).unwrap().end()..];
    let states = content_regex
        .captures_iter(rest_of_input)
        .map(|c| {
            State {
                if_0: Action {
                    write_value: c["if_0_val"].parse::<u8>().unwrap() == 1,
                    direction: c["if_0_dir"].parse().unwrap(),
                    next_state: letter_to_number(&c["if_0_state"]).unwrap(),
                },
                if_1: Action {
                    write_value: c["if_1_val"].parse::<u8>().unwrap() == 1,
                    direction: c["if_1_dir"].parse().unwrap(),
                    next_state: letter_to_number(&c["if_1_state"]).unwrap(),
                },
            }
        }).collect();

    Ok(Turing {
        current_state: initial,
        step_count,
        states,
        tape: vec![false; 100000],
        cursor: 50000
    })
}


fn main() -> Result<(), Box<Error>> {
    let input: &'static str = include_str!("input_day_25");
    let mut machine = parse(input)?;

    for _ in 0..machine.step_count {
        let action = match machine.tape[machine.cursor] {
            false => machine.states[machine.current_state].if_0,
            true => machine.states[machine.current_state].if_1,
        };
        machine.current_state = action.next_state;
        machine.tape[machine.cursor] = action.write_value;
        machine.cursor = match action.direction {
            Direction::Left => machine.cursor - 1,
            Direction::Right => machine.cursor + 1,
        }
    }

    eprintln!("tape = {:?}", machine.tape.iter().filter(|&&x| x).count());

    Ok(())
}