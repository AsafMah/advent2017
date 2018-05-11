
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
    let input: &'static str = include_str!("input_day_10");

    let inputs : Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let mut list = List::new();

    for length in inputs {
        eprintln!("list = {:?}\n", list);
        list.do_move(length);
    }

    eprintln!("list = {:?}, mul = {:?}", list, list.arr[0] * list.arr[1]);
}