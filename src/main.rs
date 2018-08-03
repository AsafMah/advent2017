extern crate failure;
extern crate itertools;
extern crate ndarray;

use itertools::Itertools;
use failure::Error;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug)]
struct Cursor {
    x: usize,
    y: usize,
    direction: Direction
}

fn advance(grid: &[Vec<u8>], cursor: &mut Cursor) {
    match cursor.direction {
        Direction::Up => cursor.x -= 1,
        Direction::Right => cursor.y += 1,
        Direction::Down => cursor.x += 1,
        Direction::Left => cursor.y -= 1,
    }
}

fn map_to_byte(c: char) -> u8 {
    match c {
        '#' => 1,
        '.' => 0,
        _ => {
            eprintln!("c = {:?}", c);
            unreachable!()
        }
    }
}

fn print_grid(grid: &[Vec<u8>], cursor: &Cursor) {
    print!("    ");
    for i in 0..grid.len() {
        print!("{:2} ", i);
    }

    println!();

    for i in 0..grid.len() {
        print!("{:02}  ", i);
        for j in 0..grid.len() {
            if cursor.x == i && cursor.y == j {
                print!("[{}]", grid[i][j]);
            } else {
                print!(" {} ", grid[i][j]);
            }
        }
        println!();
    }

    println!();

}

fn main() -> Result<(), Error> {
    let input: &'static str = include_str!("input_day_22");
    let vec = input.lines().map(|l| l.chars().map(map_to_byte).collect_vec() ).collect_vec();
    let mut grid = vec![vec!(0u8;10000);10000];
    let grid_offset = grid.len() / 2;
    for i in 0..vec.len(){
        for j in 0..vec[0].len() {
            grid[grid_offset + i][j + grid_offset] = vec[i][j];
        }
    }

    let mut virus = Cursor {
        x: grid_offset + vec[0].len() / 2,
        y: grid_offset + vec.len() / 2,
        direction: Direction::Up
    };

    let mut infected_counter = 0;

    for _ in 0..10000 {
        match grid[virus.x][virus.y] {
            1 => {
                grid[virus.x][virus.y] = 0;
                virus.direction = virus.direction.right();
            }
            0 => {
                grid[virus.x][virus.y] = 1;
                infected_counter += 1;
                virus.direction = virus.direction.left();
                eprintln!("virus = {:?}", virus);
            }
            _ => unreachable!()
        }

        advance(&grid, &mut virus);
        eprintln!("virus = {:?}", virus);
    }
    eprintln!("infected_counter = {:?}", infected_counter);
    Ok(())
}