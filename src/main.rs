
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