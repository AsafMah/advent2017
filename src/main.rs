use std::collections::HashSet;

fn main() {
    let a: &'static str = include_str!("input_day_4");
    let res = a.lines().filter(|&line| {
        let mut a = HashSet::new();
        for word in line.split_whitespace() {
            let mut mid = word.to_string().into_bytes();
            mid.sort();
            if !a.insert(mid) {
                return false;
            }
        }

        true
        
    }).count();

    eprintln!("res = {:?}", res);
}