use std::iter::zip;

fn main() {
    let input = include_str!("../../data/input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn f(t: usize, n: usize) -> Option<usize> {
    if t > n {
        Some((t - n) * n)
    } else {
        None
    }
}

fn part2(input: &str) -> usize {
    let mut input = input.lines();

    let time: usize = input
        .next()
        .expect("wtf no time")
        .split(":")
        .last()
        .expect("no times")
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .concat()
        .parse::<usize>()
        .unwrap();

    let dist: usize = input
        .next()
        .expect("wtf no time")
        .split(":")
        .last()
        .expect("no times")
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .concat()
        .parse::<usize>()
        .unwrap();

    let mut cnt = 0;
    for n in 1..time {
        if let Some(val) = f(time, n) {
            if n % 100000 == 0 {
                println!("{dist}::{val}=>({time}-{n})*{n}");
            }
            if val > dist {
                cnt += 1;
            }
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_solution_is() {
        let input = include_str!("../../data/test_input1.txt");
        let result = part2(input);
        assert_eq!(result, 71503);
    }
}
