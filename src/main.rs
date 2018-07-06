extern crate failure;
extern crate itertools;

use failure::Error;
use std::iter::{once, repeat};
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}


fn advance(grid: &Vec<Vec<char>>, x: usize, y: usize, direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Right => (x + 1, y),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
    }
}

fn idx(grid: &Vec<Vec<char>>, (x, y): (usize, usize)) -> char {
    return grid.get(y).and_then(|g| g.get(x)).map(|x| *x).unwrap_or(' ');
}

fn main() -> Result<(), Error> {
    let input: &'static str = include_str!("input_day_19");
    let directions = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
    let mut direction = Direction::Down;
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let max_line_length = grid.iter().map(|v| v.len()).max().unwrap();
    grid.push(repeat(' ').take(max_line_length).collect_vec());

    let mut x = grid[0].iter().position(|&f| f == '|').unwrap();
    let mut y = 0;
    let mut counter = 0;
    eprintln!("grid = {:#?}", grid);

    loop {
        let a = advance(&grid, x, y, direction);
        x = a.0;
        y = a.1;
        counter += 1;

        match idx(&grid, (x,y)) {
            '+' => {
                let mut dir = directions.iter().filter(|&&x| x != direction.opposite()).find(|&&d| {
                    let res = idx(&grid, advance(&grid, x, y, d));
                    return res != ' ';
                });

                match dir {
                    Some(&d) => direction = d,
                    None => break
                }
            },
            ' ' => {
                eprintln!("counter = {:?}", counter);
                return Ok(())
            },
            l @ 'A'...'Z' => eprintln!("letter {:?}", (x,y,l)),
            _ => {}
        }
    }

    Ok(())
}