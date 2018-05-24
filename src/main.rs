use std::collections::HashMap;

fn main() {
    let input: &'static str = include_str!("input_day_13");

    let data: HashMap<usize, usize> = input.lines()
        .map(|l| {
            let mut a = l.split(": ");
            (a.next().unwrap().parse().unwrap(), a.next().unwrap().parse().unwrap())
        })
        .collect();

    for i in 0.. {
        let sum = data.iter().fold(0, |acc, (&depth, &range)| {
            let pos = (depth + i) % ((range - 1) * 2);
            acc + if pos == 0  {
                //(depth * range)
                1
            } else {
                0
            }
        });

        if sum == 0 {
            eprintln!("data = {:?}", i);
            break;
        }
    }


}

fn knot_hash(input: &[u8]) -> Vec<u8> {
    let input: Vec<_> = input.iter().chain(&[17u8, 31, 73, 47, 23]).collect();
    let mut list = List::new();
    for &&length in input.iter().cycle().take(64 * input.len()) {
        list.do_move(length as usize);
    }
    list.arr
        .chunks(16)
        .map(|chunk|
            chunk.iter().fold(0u8, |acc, &i| acc ^ (i as u8)))
        .collect()
}