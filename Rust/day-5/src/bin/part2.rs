#![feature(slice_take)]
use indicatif::ProgressIterator;
use rayon::prelude::*;
use std::ops::Range;

fn main() {
    let input = include_str!("../../data/input2.txt");
    let output = part2(input);
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
    fn rev_map(&self, seed: usize) -> Option<usize> {
        if self.destination.contains(&seed) {
            let mv = seed - self.destination.start + self.source.start;
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
            .into_par_iter()
            .map(|seed| {
                if let Some(val) = self.maps.iter().filter_map(|map| map.map(seed)).last() {
                    val
                } else {
                    seed
                }
            })
            .collect()
    }
    fn rev_map(&self, seed: usize) -> usize {
        if let Some(val) = self
            .maps
            .iter()
            .rev()
            .filter_map(|map| map.rev_map(seed))
            .last()
        {
            val
        } else {
            seed
        }
    }
}

fn part2(input: &str) -> usize {
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

    let chunks = seeds.chunks(2);

    let seeds: Vec<Range<usize>> = chunks.map(|chnk| (chnk[0]..(chnk[1] + chnk[0]))).collect();

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

    let mut awnser = 0usize;
    for min in 0usize.. {
        if min % 100000 == 0 {
            println!("testing {min}:");
        }
        let out = maps.iter().fold(min, move |acc, e| {
            let a = acc.clone();
            let out = e.rev_map(acc);
            if min % 100000 == 0 {
                println!("testing {a} => {out}");
            }
            out
        });

        if let Some(_val) = seeds.iter().filter(|r| r.contains(&out)).last() {
            awnser = min;
            break;
        }
    }
    awnser
}

#[cfg(test)]
mod tests {
    use super::*;
    #[ignore]
    #[test]
    fn the_solution_is() {
        let input = include_str!("../../data/test_input1.txt");
        let result = part2(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn rev_test() {
        // 6 => 15 => 30
        let maps = vec![
            Mapper::new(
                "a".to_string(),
                "b".to_string(),
                vec![Map::new(10, 0, 10), Map::new(50, 40, 10)],
            ),
            Mapper::new(
                "b".to_string(),
                "c".to_string(),
                vec![Map::new(15, 5, 10), Map::new(20, 50, 10)],
            ),
        ];
        let zaps = vec![
            Mapper::new(
                "a".to_string(),
                "b".to_string(),
                vec![Map::new(10, 0, 10), Map::new(50, 40, 10)],
            ),
            Mapper::new(
                "b".to_string(),
                "c".to_string(),
                vec![Map::new(15, 5, 10), Map::new(20, 50, 10)],
            ),
        ];

        let seed = 42usize;

        let map = maps.into_iter().fold(vec![seed], |acc, map| {
            let input = acc.clone();
            let out = map.map(input);
            println!("map: {:?}", &out);
            out
        });

        let result = zaps.into_iter().fold(map[0], |acc, map| {
            let input = acc.clone();
            let out = map.rev_map(input);
            dbg!(&out);
            out
        });

        println!("map: {} => {:?} =<> {}", &seed, &map, &result);
        assert_eq!(result, seed);
    }
}
