#[macro_use]
extern crate combine;

use std::iter::Iterator;
use std::fs;
use std::collections::HashMap;

use combine::{
    Parser,
    parser,
    Stream,
    many1,
    parser::char::{
        alpha_num,
        char,
        spaces
    }
};

#[derive(Debug)]
struct Orbit {
    left: String,
    right: String,
}

parser!{
    fn orbit_rule[I]()(I) -> Orbit where [I: Stream<Item = char>] {
        (
            many1::<String, _>(alpha_num()),
            char(')'),
            many1::<String, _>(alpha_num()),
        ).map(|(a, _, b)| { Orbit { left: a, right: b } })
    }
}

fn path_iter<'a>(parents: &'a HashMap<&'a str, &'a str>, item: &'a str) -> impl 'a + Iterator<Item = &'a str> {
    std::iter::successors(Some(item), move |&x| {
        parents.get(x).cloned()
    }).skip(1)
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("inputs/day06.txt")?;
    let mut parser = many1(orbit_rule().skip(spaces()));
    let s: &str = &input;
    let orbits: Vec<Orbit> = match parser.easy_parse(s) {
        Ok(x) => x.0,
        Err(e) => {
            println!("Parse failed: {}", e);
            return Ok(());
        }
    };

    let parent_relations: HashMap<&str, &str> =
        orbits.iter()
        .map(|x| (&x.right as &str, &x.left as &str))
        .collect();

    let part1: usize = orbits.iter().map(|orbit| path_iter(&parent_relations, &orbit.right).count()).sum();
    println!("{}", part1);

    let santa_path: HashMap<_, _> = path_iter(&parent_relations, "SAN")
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();

    let distance_to_santa = path_iter(&parent_relations, "YOU")
        .enumerate()
        .flat_map(|(i, x)| santa_path.get(x).map(|j| i + j))
        .next();
    println!("{}", distance_to_santa.unwrap());

    Ok(())
}
