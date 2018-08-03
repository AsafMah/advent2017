extern crate failure;
extern crate itertools;

use failure::Error;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct Outlet(u32, u32);

impl Outlet {
    fn connects(self, i: u32) -> Option<(u32, u32)>{
        if self.0 == i {
            Some((self.0, self.1))
        }
        else if self.1 == i {
            Some((self.1, self.0))
        }
        else {
            None
        }
    }
}

fn strongest_bridge(current: u32, v: &Vec<Outlet>) -> u32{
    v.iter().enumerate().filter_map(|(i, o)| o.connects(current).map(|s| (i, s.0, s.1)))
    .map(|(i, new_current, next)| {
        let mut clone = v.clone();
        clone.remove(i);
        current + new_current + strongest_bridge(next, &clone)
    }).max().unwrap_or(current)
}

fn main() -> Result<(), Box<Error>> {
    let input: &'static str = include_str!("input_day_24");

    let vec = input.lines().map(|l| {
        let mut split = l.split("/");
        Outlet(split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())
    }).collect_vec();

    ;
    eprintln!("vec = {:?}", strongest_bridge(0, &vec));

    Ok(())
}