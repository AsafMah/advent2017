#[macro_use]
extern crate nom;

use std::char;
use std::str::{FromStr, from_utf8};
use nom::{digit, types::CompleteByteSlice};

named!(letter_to_num<CompleteByteSlice, usize>,
    map!(take!(1), |b| (b[0] as usize) - ('a' as usize) )
);

named!(number<CompleteByteSlice, usize>,
    map_res!(map_res!(map!(digit, |b| b.0), from_utf8), FromStr::from_str)
);

named!(parse_move<CompleteByteSlice, Move>,
alt!(
    do_parse!(
    char!('s') >>
    count: number >>
    (Move::Spin(count))
    ) |
    do_parse!(
    char!('x') >>
    first: number >>
    char!('/') >>
    second: number >>
    (Move::Exchange(first, second))
    ) |
    do_parse!(
    char!('p') >>
    first: letter_to_num >>
    char!('/') >>
    second: letter_to_num >>
    (Move::Partner(first, second))
    )
    )
);


named!(parse_moves<CompleteByteSlice, Vec<Move>>,
    separated_list!(char!(',') , parse_move)
);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(usize, usize),
}

fn main()  {
    let input: &'static str = include_str!("input_day_16.txt");
    let mut state: Vec<_> = (0..16).collect();
    let mut states = vec![state.clone()];
    eprintln!("state = {:?}", state);
    let data2 = parse_moves(CompleteByteSlice(input.as_bytes()));
    let data2 = data2.unwrap().1;

    for _ in 0..1_000_000_000 {
        for dance_move in &data2 {
            match dance_move {
                Move::Spin(count) => state.rotate_right(*count),
                Move::Exchange(i1, i2) => state.swap(*i1, *i2),
                Move::Partner(p1, p2) => {
                    let pos1 = state.iter().position(|c| c == p1).unwrap();
                    let pos2 = state.iter().position(|c| c == p2).unwrap();
                    state.swap(pos1, pos2)
                }
            };
        }

        if states[0] == state {
            break;
        }

        states.push(state.clone());

    }

    let states_strings = states.iter().map(|s| s.iter().map(|c| char::from_u32((c + 97) as u32).unwrap()).collect::<String>()).collect::<Vec<_>>();

    eprintln!("data = {:?}", states_strings);
    eprintln!("1_000_000_000 % states.len() = {:?}", states_strings[1_000_000_000 % states.len()]);
}