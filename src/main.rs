#[derive(Debug)]
struct List {
    arr: Vec<u32>,
    index: usize,
    skip: usize,
}

impl List {
    fn new() -> Self {
        Self {
            arr: (0..=255).collect(),
            index: 0,
            skip: 0,
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

fn u8_to_bit_array(n: u8) -> [bool; 8] {
    let mut res = [false; 8];
    let mut current = n;
    for i in (0..res.len()).rev() {
        res[i] = (current & 1) == 1;
        current >>= 1;
    }
    res
}

fn walk(vec: &mut Vec<Vec<bool>>, index: (usize, usize)) {
    if vec[index.0][index.1] == false {
        return;
    }
    vec[index.0][index.1] = false;
    if index.0 + 1 != vec.len() {
        let new_index = (index.0 + 1, index.1);
        walk(vec, new_index);
    }
    if index.0  != 0 {
        walk(vec, (index.0 - 1, index.1));
    }
    if index.1 + 1 != vec.len() {
        walk(vec, (index.0, index.1 + 1));
    }
    if index.1  != 0 {
        walk(vec, (index.0, index.1 - 1));
    }
}

fn main() {
    let input: &'static str = "amgozmfv";
    let mut vec = (0..128).map(|i| {
        let line = format!("{}-{}", input, i);
        knot_hash(line.as_bytes())
    })
        .map(|vec| vec.iter().flat_map(|&b| u8_to_bit_array(b).to_vec()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut counter = 0;

    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            if vec[i][j] {
                counter += 1;
                walk(&mut vec, (i, j));
            }
        }
    }


    eprintln!("counter = {:?}", counter);
}

fn knot_hash(input: &[u8]) -> Vec<u8> {
    let input: Vec<_> = input.iter().chain(&[17u8, 31, 73, 47, 23]).collect();
    let mut list = List::new();
    for &&length in input.iter().cycle().take(64 * input.len()) {
        list.do_move(length as usize);
    }
    list.arr
        .chunks(16)
        .map(|chunk|
            chunk.iter().fold(0u8, |acc, &i| acc ^ (i as u8)))
        .collect()
}