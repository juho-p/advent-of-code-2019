use itertools::Itertools;

fn next(n: u32) -> u32 {
    if n % 10 == 9 {
        let a = n / 10;
        let b = next(a);
        b * 10 + (b % 10)
    } else {
        n + 1
    }
}

fn rdigits(n: u32) -> impl Iterator<Item = u32> {
    let mut x = n;
    std::iter::from_fn(move || {
        if x > 0 {
            let r = x % 10;
            x /= 10;
            Some(r)
        } else {
            None
        }
    })
}

fn is_valid_part1(n: u32) -> bool {
    rdigits(n).tuple_windows().any(|(a, b)| a == b)
}

fn is_valid_part2(n: u32) -> bool {
    let groups: Vec<_> = rdigits(n)
        .tuple_windows::<(_,_,_)>()
        .filter(|(a, b, c)| a == b && b == c)
        .map(|t| t.0)
        .collect();

    rdigits(n).tuple_windows()
        .filter(|(a, b)| a == b)
        .any(|(a, _)| !groups.contains(&a))
}

fn main() {
    let input: (u32, u32) = (188888,657474);

    let attempts =
        std::iter::successors(
            Some(input.0),
            |&n| {
                let m = next(n);
                if m < input.1 { Some(m) } else { None }
            });

    let part1: Vec<u32> = attempts.filter(|n| is_valid_part1(*n)).collect();
    println!("part1 {}", part1.len());

    println!("part1 {}", part1.iter().filter(|x| is_valid_part2(**x)).count());
}
