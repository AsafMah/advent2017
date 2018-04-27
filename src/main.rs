fn main() {
    let input : &'static str = include_str!("input_day_5");
    let mut vec : Vec<i32>  = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut i : i32 = 0;
    let mut counter = 0;
    loop {
        let ins = vec[i as usize];
        vec[i as usize] = if ins >= 3 { vec[i as usize] - 1} else { vec[i as usize] + 1 } ;
        i += ins;
        counter += 1;

        if i < 0 || i >= (vec.len() as i32) {
            eprintln!("counter = {:?}", counter);
            break;
        }
    }
}