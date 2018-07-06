extern crate failure;
extern crate itertools;
#[macro_use]
extern crate nom;
extern crate multimap;

use failure::Error;
use nom::types::CompleteStr;
use nom::line_ending;
use multimap::MultiMap;
use std::collections::HashMap;

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
    let particles : Vec<Particle> = particles(CompleteStr(input)).unwrap().1;
    let mut particles : HashMap<usize, Particle> = particles.into_iter().enumerate().collect();

    for _ in 0..10000 {
        let mut multimap = MultiMap::new();

        for (i, particle) in particles.iter_mut() {
            particle.do_step();
            multimap.insert(particle.pos, *i);
        }

        for (_, values) in multimap.iter_all() {
            if values.len() == 1 {
                continue;
            }

            for value in values {
                particles.remove(value);
            }
        }

    }

    eprintln!("particles.len() = {:?}", particles.len());
    Ok(())
}