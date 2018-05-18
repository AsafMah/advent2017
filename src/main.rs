use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    North,
    South,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
}

impl Direction {
    fn parse(str: &str) -> Direction {
        match str {
            "n" => Direction::North,
            "s" => Direction::South,
            "ne" => Direction::NorthEast,
            "se" => Direction::SouthEast,
            "nw" => Direction::NorthWest,
            "sw" => Direction::SouthWest,
            _ => panic!("np")
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(Self::parse(s))
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
struct Grid {
    x: i32,
    y: i32,
}

impl Grid {
    fn next(&mut self, dir: Direction) {
        match dir {
            Direction::North => {self.y += 2;},
            Direction::South => {self.y -= 2;},
            Direction::NorthEast => {self.y += 1;self.x += 1;},
            Direction::SouthEast => {self.y -= 1;self.x += 1;},
            Direction::NorthWest => {self.y += 1;self.x -= 1;},
            Direction::SouthWest => {self.y -= 1;self.x -= 1;},
        }
    }

    fn smart_step(&mut self) {
        let dir = if self.y.abs() == self.x.abs() {
            match (self.x.signum(), self.y.signum()) {
                (1, 1) => Direction::SouthWest,
                (1, -1) => Direction::NorthWest,
                (-1, 1) => Direction::SouthEast,
                (-1, -1) => Direction::NorthEast,
                _ => panic!("at the disco")
            }
        }
        else {
            if self.y > self.x {
                Direction::South
            }
            else {
                Direction::North
            }
        };

        self.next(dir)
    }
}


fn main() {
    let input: &'static str = include_str!("input_day_11");
    let dirs : Vec<Direction> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut counts = Vec::new();
    let mut grid = Grid{x: 0, y: 0};
    for dir in dirs {
        grid.next(dir);

        let mut test_grid = grid.clone();
        let mut counter = 0;

        while test_grid.x != 0 || test_grid.y != 0 {
            test_grid.smart_step();
            counter += 1;
        }

        counts.push(counter);
    }
    

    eprintln!("grid = {:?}", counts.iter().max());
}