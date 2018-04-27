extern crate regex;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Listing {
    name: String,
    weight: u32,
    children: Vec<String>,
}

use regex::Regex;
use std::iter::empty;
use std::collections::HashSet;
use std::collections::HashMap;


fn calc_weight(lst: &Listing, map: &HashMap<&str, &Listing>) -> (u32, u32)
{
    if  lst.children.len() == 0 {
        return (lst.weight, 0);
    }

    let child_weights = lst.children.iter()
        .map(|c| calc_weight(map[c.as_str()], map))
        .collect::<Vec<_>>();
    let weight = child_weights[0];

    match child_weights.iter().find(|&&w| w.0 + w.1 != weight.0 + weight.1) {
        Some((wrong, child)) => {
            eprintln!("Wrong weight {:?} {:?}, should be {:?}" , wrong, child, ((weight.0 + weight.1) - child ));
            (*wrong, 0)
        }
        None => {
            eprintln!("weight = {:?}, children = {:?}", lst.weight, child_weights);
            (lst.weight, child_weights.iter().map(|(a,b)| a + b).sum::<u32>())
        }
    }
}

fn main() {
    let input: &'static str = include_str!("input_day_7");
    let regex = Regex::new(r"(?P<Name>[a-z]+)\s\((?P<Weight>\d+)\)( -> (?P<Children>.*))?").unwrap();

    let matches = input.lines().map(|line| {
        let cap = regex.captures(line).unwrap();
        Listing {
            name: cap.name("Name").unwrap().as_str().to_string(),
            weight: cap.name("Weight").unwrap().as_str().parse().unwrap(),
            children: cap.name("Children").map(|s| s.as_str().split(", ").map(|s| s.to_string()).collect::<Vec<_>>()).unwrap_or(Vec::new())
        }
    }).collect::<Vec<_>>();

    let set = matches.iter().flat_map(|x| x.children.iter()).collect::<HashSet<_>>();
    let res = matches.iter().find(|x| !set.contains(&x.name)).unwrap();
    let map = matches.iter().map(|x| (x.name.as_str(), x)).collect::<HashMap<_,_>>();

    eprintln!("matches = {:#?}", res);
    calc_weight(res, &map);

}