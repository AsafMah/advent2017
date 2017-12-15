fn main() {
    //day_one();
    //day_two();
    day_two_2();
}

fn day_one() {
    let data = include_bytes!("input_day_one.txt");
    let sum = data
        .iter()
        .zip(data.iter().cycle().skip(data.len() / 2))
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

fn day_two() {
    let data: &str = include_str!("input_day_two.txt");
    let result = data.split('\n')
        .map(|l: &str| {
            let (min, max) = l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .fold((std::i32::MAX, std::i32::MIN), |(min, max), i| {
                    (if i < min { i } else { min },
                     if i > max { i } else { max })
                });
            max - min
        })
        .sum::<i32>();
    eprintln!("result = {:?}", result);
}

fn day_two_2() {
    let data: &str = include_str!("input_day_two.txt");
    let result = data.split('\n')
        .map(|l: &str| {
            let line : Vec<i32> = l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap()).collect();
            line.iter().fold(None, |o, &i| {
                    o.or_else(|| {
                        line.iter().find(|&&x| i != x && i % x == 0).map(|x| i / x)
                    })
                }).unwrap()
        })
        .sum::<i32>();
    eprintln!("result = {:?}", result);
}