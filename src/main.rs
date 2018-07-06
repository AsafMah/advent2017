extern crate failure;
extern crate itertools;
#[macro_use]
extern crate nom;

use failure::Error;
use nom::types::CompleteStr;
use nom::line_ending;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn texicab_distance(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}


#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3
}

impl Particle {
    fn distance(&self) -> i64 {
        self.pos.texicab_distance()
    }

    fn do_step(&mut self) {
        self.vel.x += self.acc.x;
        self.vel.y += self.acc.y;
        self.vel.z += self.acc.z;
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
}


fn is_number(chr: char) -> bool {
    chr >= '0' && chr <= '9' || chr == '-'
}

named!(number<CompleteStr, i64>,
    flat_map!(take_while!(is_number), parse_to!(i64))
);

named!(vec3<CompleteStr, Vec3>,
    do_parse!(
        tag!("<") >>
        x: number >>
        tag!(",") >>
        y: number >>
        tag!(",") >>
        z: number >>
        tag!(">") >>
        (Vec3 { x, y, z})
    )
);


named!(particle<CompleteStr, Particle>,
    do_parse!(
        tag!("p=") >>
        pos: vec3 >>
        tag!(", v=") >>
        vel: vec3 >>
        tag!(", a=") >>
        acc: vec3 >>
        (Particle { pos, vel, acc})
    )
);

named!(particles<CompleteStr, Vec<Particle>>,
    separated_list!(line_ending , particle)
);

fn main() -> Result<(), Error> {
    let input : &'static str = include_str!("input_day_20");
    let mut particles : Vec<Particle> = particles(CompleteStr(input)).unwrap().1;

    for _ in 0..10000 {
        for particle in particles.iter_mut() {
            particle.do_step()
        }
    }

    eprintln!("particles.iter().max_by() = {:?}", particles.iter().enumerate().min_by_key(|&(_, item)| item.distance()));
    Ok(())
}