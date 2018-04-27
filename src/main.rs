#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
struct Point(isize, isize);

impl Point {
    fn advance(&self, d: Direction) -> Point {
        let (fst, snd) = match d {
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, -1),
        };

        Point(self.0 + fst, self.1 + snd)
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Board(Vec<u32>);

impl Board {
    const SIZE: usize = 700;

    fn new() -> Board {
        Board(vec![0 as u32; Self::SIZE * Self::SIZE])
    }

    fn normalize_point(p: Point) -> usize
    {
        let x = ((p.0 + (Self::SIZE as isize)) as usize) % Self::SIZE;
        let y = ((p.1 + (Self::SIZE as isize)) as usize) % Self::SIZE;

        y * Self::SIZE + x
    }

    fn get(&self, p: Point) -> u32 {
        self.0[Self::normalize_point(p)]
    }

    fn set(&mut self, p: Point, v: u32) {
        self.0[Self::normalize_point(p)] = v;
    }

    fn adjacent_sum(&self, p: Point) -> u32 {
        let mut sum = 0;
        for d in &[Direction::Up, Direction::Down] {
            let point = p.advance(*d);
            sum += self.get(point);
            sum += self.get(point.advance(Direction::Left));
            sum += self.get(point.advance(Direction::Right));
        }
        sum += self.get(p.advance(Direction::Left));
        sum += self.get(p.advance(Direction::Right));

        sum
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn next(&self) -> Direction {
        match *self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }
}


fn main() {
    let mut a = Point::default();
    let mut board = Board::new();
    let mut direction = Direction::Down;

    board.set(a, 1);

    for i in 2..300000 {
        let next = direction.next();
        let natural_next = a.advance(next);

        a = if board.get(natural_next) != 0 {
            a.advance(direction)
        } else {
            direction = next;
            natural_next
        };

        let v = board.adjacent_sum(a);

        board.set(a, v);

        if v > 289326 {
            eprintln!("a = {:?}", v);
            break;
        }
    }

}