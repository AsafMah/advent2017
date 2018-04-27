use std::collections::HashSet;

fn main() {
    let input: &'static str = include_str!("input_day_6");
    let mut vec: Vec<u32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut a = HashSet::new();
    let mut i = 0;
    let mut golden_state : Option<Vec<_>> = None;
    loop {
        eprintln!("vec = {:?}", (i, &vec));
        match golden_state {
            None => {
                if !a.insert(vec.clone()) {
                    golden_state = Some(vec.clone());
                    i = 0;
                }
            }
            Some(ref v) => {
                if (vec == *v)
                    {
                        eprintln!("i = {:?}", i);
                        break;
                    }
            }
        }

        i += 1;

        let (max_index, max) = vec.iter().enumerate().fold((0, 0), |(max_ind, max), (ind, &elem)| {
            if elem > max {
                (ind, elem)
            } else {
                (max_ind, max)
            }
        });

        vec[max_index] = 0;

        let mut index = ((max_index + 1) % vec.len()) as usize;
        let mut current = max;

        loop {
            vec[index] += 1;
            current -= 1;
            if current <= 0 {
                break;
            }
            index = (index + 1) % vec.len();
        }
    }
}