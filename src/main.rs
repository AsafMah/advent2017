const DATA: &[u8] = include_bytes!("input.txt");

fn main() {
    day_one();
}

fn day_one() {
    let sum = DATA
        .iter()
        .zip(DATA.iter().cycle().skip(DATA.len() / 2))
        .fold(0u32, |acc, (&cur, &next)| {
            let (cur, next) = ((cur as char).to_digit(10).unwrap(), (next as char).to_digit(10).unwrap());
            eprintln!("(cur, next) = {:?}", (cur, next));
            acc + if cur == next {
                cur
            } else {
                0
            }
        });
    eprintln!("sum = {:?}", sum);
}
