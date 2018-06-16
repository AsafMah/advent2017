use std::error::Error;

fn main() -> Result<(), Box<Error>>  {
    let input: usize = include_str!("input_day_17.txt").parse()?;
    let mut buf = vec![0];
    buf.reserve(50_000_000);
    let mut position = 0;
    let mut current = 0;

    for i in 1..50_000_000 {
        position = (position + input) % i + 1;
        if  position == 1 {
            current = i;
        }
    }

    eprintln!("current = {:?}", current);

    Ok(())
}