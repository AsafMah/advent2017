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

    fn flip(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug)]
struct Cursor {
    x: usize,
    y: usize,
    direction: Direction
}

impl Cursor {
    fn turn_from_state(&mut self, state: NodeState) {
        self.direction = match state {
            NodeState::Clean => self.direction.left(),
            NodeState::Weakened => self.direction,
            NodeState::Infected => self.direction.right(),
            NodeState::Flagged => self.direction.flip(),
        }
    }

    fn advance(&mut self) {
        match self.direction {
            Direction::Up => self.x -= 1,
            Direction::Right => self.y += 1,
            Direction::Down => self.x += 1,
            Direction::Left => self.y -= 1,
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


#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged
}

impl NodeState {
    fn from_char(c : char) -> Self {
        match c {
            '#' => NodeState::Infected,
            '.' => NodeState::Clean,
            _ => {
                eprintln!("c = {:?}", c);
                unreachable!()
            }
        }
    }

    fn next_state(self) -> Self{
        match self {
            NodeState::Clean => NodeState::Weakened,
            NodeState::Weakened => NodeState::Infected,
            NodeState::Infected => NodeState::Flagged,
            NodeState::Flagged => NodeState::Clean,
        }
    }
}

fn main() -> Result<(), Error> {
    let input: &'static str = include_str!("input_day_22");
    let vec = input.lines().map(|l| l.chars().map(NodeState::from_char).collect_vec() ).collect_vec();
    let mut grid = vec![vec!(NodeState::Clean;10000);10000];
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

    for _ in 0..10000000 {
        let current = grid[virus.x][virus.y];

        virus.turn_from_state(current);
        grid[virus.x][virus.y] = current.next_state();
        if current.next_state() == NodeState::Infected {
            infected_counter += 1;
        }

        virus.advance();
    }
    eprintln!("infected_counter = {:?}", infected_counter);
    Ok(())
}