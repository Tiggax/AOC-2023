#![feature(slice_take)]
use std::ops::Range;

fn main() {
    let input = include_str!("../../data/input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Map {
    source: Range<usize>,
    destination: Range<usize>,
}
impl Map {
    fn new(dest: usize, source: usize, num: usize) -> Self {
        Map {
            source: source..(source + num),
            destination: dest..(dest + num),
        }
    }
    fn map(&self, seed: usize) -> Option<usize> {
        if self.source.contains(&seed) {
            let mv = seed - self.source.start + self.destination.start;
            Some(mv)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Mapper {
    from: String,
    to: String,
    maps: Vec<Map>,
}

impl Mapper {
    fn new(from: String, to: String, maps: Vec<Map>) -> Self {
        Mapper { from, to, maps }
    }
    fn map(&self, seeds: Vec<usize>) -> Vec<usize> {
        seeds
            .into_iter()
            .map(|seed| {
                if let Some(val) = self.maps.iter().filter_map(|map| map.map(seed)).last() {
                    val
                } else {
                    seed
                }
            })
            .collect()
    }
}

fn part1(input: &str) -> usize {
    let mut input = input.lines();
    let seeds: Vec<usize> = input
        .next()
        .expect("wtf?")
        .split(":")
        .last()
        .expect("no seeds? T.T")
        .trim()
        .split_whitespace()
        .map(|nm| nm.parse::<usize>().unwrap())
        .collect();

    let maps: Vec<Mapper> = input
        .collect::<Vec<&str>>()
        .split(|&s| s.is_empty())
        .filter(|&chunk| !chunk.is_empty())
        .map(|chunk| {
            let mut chunk = chunk;
            let st = chunk.take_first().expect("should have chunk");
            let names: Vec<&str> = st
                .split_whitespace()
                .next()
                .expect("ni space?")
                .split("-")
                .collect();
            let mut maps: Vec<Map> = Vec::new();

            while let Some(chnk) = chunk.take_first() {
                let args: Vec<usize> = chnk
                    .split_whitespace()
                    .map(|nm| nm.parse().unwrap())
                    .collect();
                // destination source num
                maps.push(Map::new(args[0], args[1], args[2]))
            }

            Mapper::new(names[0].to_owned(), names[2].to_owned(), maps)
        })
        .collect();

    dbg!(&maps);
    dbg!(&seeds);

    let seeds = maps.into_iter().fold(seeds, move |acc, e| {
        let a = acc.clone();
        let out = e.map(acc);

        println!("folding: {:?} => {:?}", &a, &out);
        out
    });
    dbg!(&seeds);
    if let Some(val) = seeds.iter().min() {
        *val
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_solution_is() {
        let input = include_str!("../../data/test_input1.txt");
        let result = part1(input);
        assert_eq!(result, 35);
    }
}
