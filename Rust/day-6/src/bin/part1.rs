use std::iter::zip;

fn main() {
    let input = include_str!("../../data/input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn f(t: usize, n: usize) -> Option<usize> {
    if t > n {
        Some((t - n) * n)
    } else {
        None
    }
}

fn part1(input: &str) -> usize {
    let mut input = input.lines();

    let time: Vec<usize> = input
        .next()
        .expect("wtf no time")
        .split(":")
        .last()
        .expect("no times")
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    let dist: Vec<usize> = input
        .next()
        .expect("wtf no time")
        .split(":")
        .last()
        .expect("no times")
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    let mut races = zip(time, dist);
    dbg!(&races);

    let res: usize = races
        .map(|(t, d)| {
            let mut cnt = 0;
            for n in 1..t {
                if let Some(val) = f(t, n) {
                    println!("{d}::{val}=>({t}-{n})*{n}");
                    if val > d {
                        cnt += 1;
                    }
                }
            }
            cnt
        })
        .product();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_solution_is() {
        let input = include_str!("../../data/test_input1.txt");
        let result = part1(input);
        assert_eq!(result, 288);
    }
}
