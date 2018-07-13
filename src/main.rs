extern crate failure;
extern crate itertools;
extern crate ndarray;

use failure::Error;
use std::collections::HashMap;
use ndarray::Array2;

type Slot = u8;
type GridHash = Vec<Slot>;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Hash)]
struct Grid(Vec<Vec<Slot>>);

impl Grid {
    fn hash(&self) -> GridHash {
        let mut sum_vec = Vec::new();

        for v in self.0.iter() {
            sum_vec.push(v.iter().sum());
        }

        let mut sum_top_left_to_bottom_right = 0;
        let mut sum_top_right_to_bottom_left = 0;

        for i in 0..self.0.len() {
            let mut sum = 0;
            for j in 0..self.0[i].len() {
                sum += self.0[j][i];
            }
            sum_top_left_to_bottom_right += self.0[i][i];
            sum_top_right_to_bottom_left += self.0[self.0.len() - 1 - i][self.0.len() - 1 - i];
            sum_vec.push(sum);
        }
        sum_vec.push(sum_top_left_to_bottom_right);
        sum_vec.push(sum_top_right_to_bottom_left);

        sum_vec.sort();
        sum_vec
    }

    fn enrich(&mut self, pattern_map: &HashMap<GridHash, Grid>) {
        let chunk_size = if self.0.len() % 2 == 0 { 2 } else { 3 };

        for i in (0..self.0.len()).step_by(chunk_size) {
            for j in (0..self.0[i].len()).step_by(chunk_size) {
                let grid = Grid(self.0[i..(i+chunk_size)].iter().map(|v| v[j..(j+chunk_size)].into()).collect());
                let new_grid = &pattern_map[&grid.hash()];
                eprintln!("new_grid = {:?}", new_grid);
            }
        }
    }
}

fn map_to_byte(c: char) -> Slot {
    match c {
        '#' => 1,
        '.' => 0,
        _ => {
            eprintln!("c = {:?}", c);
            unreachable!()
        }
    }
}

fn main() -> Result<(), Error> {
    let input: &'static str = include_str!("input_day_21");

    let mut initial_grid = Grid(
    vec![
        vec![0, 1, 0],
        vec![0, 0, 1],
        vec![1, 1, 1],
    ]);


    let mut pattern_map: HashMap<_, _> =
        input.lines().map(|l| {
            let mut parts = l.split(" => ");
            let key = Grid(parts.next().unwrap().split("/").map(|l| l.chars().map(map_to_byte).collect()).collect());
            let val = Grid(parts.next().unwrap().split("/").map(|l| l.chars().map(map_to_byte).collect()).collect());

            (key.hash(), val)
        }).collect();

    initial_grid.enrich(&pattern_map);

    eprintln!("g = {:?}", pattern_map);
    Ok(())
}