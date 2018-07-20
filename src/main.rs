extern crate failure;
extern crate itertools;
extern crate ndarray;

use failure::Error;
use ndarray::Array2;
use ndarray::prelude::arr2;
use std::collections::HashMap;
use itertools::Itertools;
use ndarray::Axis;
use ndarray::stack;

type Slot = u8;
type PatternMap = HashMap<Array2<Slot>, Grid>;


fn find_in_map_reverse_transpose<'a>(map: &'a PatternMap, arr: &Array2<Slot>) -> Result<&'a Grid, Array2<Slot>> {
    let mut arr = arr.clone().reversed_axes();

    map.get(&arr).or_else(|| {
        arr.invert_axis(Axis(1));
        map.get(&arr)
    }).ok_or_else(|| arr)
}

fn find_in_map<'a>(map: &'a PatternMap, arr: &Array2<Slot>) -> &'a Grid {
    find_in_map_reverse_transpose(&map, arr)
        .or_else(|arr| find_in_map_reverse_transpose(&map, &arr))
        .or_else(|arr| find_in_map_reverse_transpose(&map, &arr))
        .or_else(|arr| find_in_map_reverse_transpose(&map, &arr))
        .unwrap()
}


#[derive(Clone, Debug, Hash)]
struct Grid(Array2<Slot>);

impl Grid {
    fn enrich(&self, map: &PatternMap) -> Grid {
        let len = if self.0.len_of(Axis(0)) % 2 == 0 { 2 } else { 3 };
        let v = self.0
            .exact_chunks((len, len))
            .into_iter()
            .map(|c| find_in_map(map, &c.to_owned()).0.view())
            .collect_vec();

        let new_arrays = v.chunks(self.0.len_of(Axis(0)) / len)
            .map(|c| stack(Axis(1), c).unwrap())
            .collect_vec();

        let views = new_arrays.iter().map(|v| v.view()).collect_vec();


        Grid(stack(Axis(0), &views).unwrap())
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

    let initial_grid = Grid(
        arr2(&[
            [0, 1, 0],
            [0, 0, 1],
            [1, 1, 1],
        ]));


    let pattern_map: PatternMap =
        input.lines().map(|l| {
            let mut parts = l.split(" => ");
            let key: Vec<_> = parts.next().unwrap().split("/").flat_map(|l| l.chars().map(map_to_byte)).collect();
            let len = if key.len() == 4 { 2 } else { 3 };
            let key = Array2::from_shape_vec((len, len), key).unwrap();

            let val = parts.next().unwrap().split("/").flat_map(|l| l.chars().map(map_to_byte)).collect();
            let val = Grid(Array2::from_shape_vec((len + 1, len + 1), val).unwrap());
            (key, val)
        }).collect();

    let mut grid = initial_grid;
    for _ in 0..18 {
         grid = grid.enrich(&pattern_map);
        eprintln!("count = {:?}", grid.0.iter().fold(0u64, |acc, &i| acc + (i as u64)));
    }

    Ok(())
}