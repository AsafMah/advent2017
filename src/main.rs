use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input: &'static str = include_str!("input_day_12");

    let mut data : HashMap<usize, Vec<usize>> = input.lines().enumerate()
        .map(|(i, l)|
                 (i, l.split(" <-> ")
                .nth(1).unwrap()
                .split(", ")
                .map(|d| d.parse().unwrap())
                .collect()))
        .collect();


    let mut results_vec = Vec::new();

    loop {
        if data.is_empty() {
            break;
        }

        let mut results  = HashSet::new();

        results.insert(*data.keys().min().unwrap());

        for _ in 0..data.len() {
            for (&i, cons) in data.iter() {
                if results.contains(&i) {
                    continue;
                }
                for con in cons {
                    if results.contains(con) {
                        results.insert(i);
                        break;
                    }
                }
            }
        }

        eprintln!("results = {:?}", results);
        for result in results.iter() {
            data.remove(&result);
        }
        results_vec.push(results);
    }



    eprintln!("results = {:?}", results_vec.len());

}