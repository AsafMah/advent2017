#[derive(Debug)]
struct List {
    arr: Vec<u32>,
    index: usize,
    skip: usize
}

impl List {
    fn new() -> Self {
        Self {
            arr: (0..=255).collect(),
            index: 0,
            skip: 0
        }
    }

    fn do_move(&mut self, len: usize) {
        for i in 0..(len / 2) {
            let start_index = (self.index + i) % self.arr.len();
            let end_index = (self.index + len - i - 1) % self.arr.len();
            self.arr.swap(start_index, end_index);
        }
        self.index += len + self.skip;
        self.skip += 1;

    }
}


fn main() {
    let input: &'static [u8] = include_bytes!("input_day_10");
    let input : Vec<_> = input.iter().chain(&[17u8, 31, 73, 47, 23]).collect();

    let mut list = List::new();

    for &&length in input.iter().cycle().take(64 * input.len()) {
        list.do_move(length as usize);
    }

    let str : String = list.arr
        .chunks(16)
        .map(|chunk|
            chunk.iter().fold(0u8, |acc, &i| acc ^ (i as u8)))
        .map(|b| format!("{:02X}", b))
        .collect();

    eprintln!("str = {}", str);
}