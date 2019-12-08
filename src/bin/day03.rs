use std::collections::{HashSet, HashMap};
use std::fs;

type Dir = (i8, i8);

type Point = (i32, i32);

struct Line {
    dir: Dir,
    dist: u16,
}

fn read_path_line(s: &str) -> Line {
    let (dir_str, dist_str) = s.split_at(1);
    let dir = match dir_str {
        "U" => (0, -1),
        "R" => (1, 0),
        "D" => (0, 1),
        "L" => (-1, 0),
        _ => panic!("invalid direction"),
    };
    let dist: u16 = dist_str.parse().unwrap();

    Line {
        dir,
        dist
    }
}

fn cells(lines: &[Line]) -> Vec<Point> {
    let mut pos: Point = (0i32, 0i32);
    let mut cells = Vec::new();

    for line in lines {
        for _ in 0..line.dist {
            pos.0 += line.dir.0 as i32;
            pos.1 += line.dir.1 as i32;
            cells.push(pos);
        }
    }

    cells
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("inputs/day03.txt")?;
    let lines: Vec<Vec<_>> = input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split(',')
             .map(|line| read_path_line(line))
             .collect())
        .collect();

    let wire1 = cells(&lines[0]);
    let wire2 = cells(&lines[1]);

    let wire1_set: HashSet<_> = wire1.iter().cloned().collect();
    let wire2_set: HashSet<_> = wire2.iter().cloned().collect();
    let crossings: HashSet<_> = wire1_set
        .intersection(&wire2_set)
        .collect();

    let closest_crossing = crossings.iter().map(|x| x.0.abs() + x.1.abs()).min();
    println!("part1: {}", closest_crossing.unwrap());

    let crossing_distances = |wire: &Vec<Point>| -> HashMap<Point, usize> {
        wire.iter().enumerate()
            .filter(|(_, x)| crossings.contains(x))
            .map(|(i, x)| (*x,  i + 1))
            .collect()
    };
    let wire1_crossings = crossing_distances(&wire1);
    let wire2_crossings = crossing_distances(&wire2);

    let part2 = crossings.iter()
        .map(|p| wire1_crossings[p] + wire2_crossings[p])
        .min();
    println!("part2: {}", part2.unwrap());

    Ok(())
}
