use std::error::Error;

fn main() -> Result<(), Box<Error>>  {
    let input: usize = include_str!("input_day_17.txt").parse()?;
    let mut buf = vec![0];
    let mut current = 0;

    for i in 1..2018 {
        let new_position = (current + input) % buf.len();
        buf.insert(new_position, i);
        current = new_position + 1;
    }

    eprintln!("buf = {:?}", buf[current]);

    Ok(())
}