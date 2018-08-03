extern crate failure;

use failure::Error;

fn main() -> Result<(), Box<Error>> {
    //read input_day_23 to understand why i'm doing this
    let mut non_primes = 0;
    let mut current = 109300;
    loop {
        if current > 126300 {
            break;
        }

        let sqrt = (current as f64).sqrt() as i32 + 1;
        let mut prime = true;
        'outer: for i in 2..sqrt {
            for j in i..current {
                if i * j == current {
                    non_primes += 1;
                    prime = false;
                    break 'outer;
                }
            }

        }

        if prime {
            eprintln!("prime = {:?}", current);
        }

        current += 17;

    }
    eprintln!("current, non_primes = {:?}", (current, non_primes));

    Ok(())
}