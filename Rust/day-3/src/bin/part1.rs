use std::ops::{Range, RangeInclusive};

fn main() {
    let input = include_str!("../../data/input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
pub struct SerialNumber {
    start: usize,
    end: usize,
    row: usize,
    number: u32,
}

impl SerialNumber {
    fn new(number: u32, start: usize, end: usize, row: usize) -> Self {
        SerialNumber {
            start,
            end,
            row,
            number,
        }
    }
    fn add(&mut self, number: u32, pos: usize) -> Self {
        SerialNumber {
            start: self.start,
            end: pos,
            row: self.row,
            number: (self.number * 10 + number),
        }
    }
    fn get_range(&self, max: usize) -> RangeInclusive<usize> {
        let start = if self.start == 0 {
            self.start
        } else {
            self.start - 1
        };
        let end = if self.end + 1 == max {
            self.end
        } else {
            self.end + 1
        };

        start..=end
    }
}

fn part1(input: &str) -> u32 {
    let mut cnt = 0;
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    for (lid, line) in matrix.iter().enumerate() {
        let mut sn: Option<SerialNumber> = None;
        'inner: for (cid, chr) in line.iter().enumerate() {
            dbg!(&sn);
            if let Some(num) = chr.to_digit(10) {
                if let Some(ref mut s) = sn {
                    sn = Some(s.add(num, cid));
                } else {
                    sn = Some(SerialNumber::new(num, cid, cid, lid));
                }
            } else {
                if let Some(s) = sn {
                    // check tail of dig
                    if *chr != '.' {
                        cnt += s.number;
                        sn = None;
                        println!("add 1");
                        continue 'inner;
                    }
                    // check start of dig
                    if s.start > 0 {
                        if let Some(val) = line.get(s.start - 1) {
                            if *val != '.' && !val.is_digit(10) {
                                cnt += s.number;
                                sn = None;
                                println!("add 2 = {val}");
                                continue 'inner;
                            }
                        }
                    }
                    // check above
                    if s.row > 0 {
                        if let Some(above) = matrix.get(s.row - 1) {
                            dbg!(s.get_range(above.len()));
                            if let Some(vals) = above.get(s.get_range(above.len())) {
                                println!("3 - : {:?}", &vals);
                                if let Some(_) = vals.iter().find(|c| !c.is_digit(10) && **c != '.')
                                {
                                    cnt += s.number;
                                    sn = None;
                                    println!("add 3");
                                    continue 'inner;
                                }
                            }
                        }
                    }
                    // check bottom
                    if let Some(below) = matrix.get(s.row + 1) {
                        if let Some(vals) = below.get(s.get_range(below.len())) {
                            if let Some(_) = vals.iter().find(|c| !c.is_digit(10) && **c != '.') {
                                cnt += s.number;
                                sn = None;
                                println!("add 4");
                                continue 'inner;
                            }
                        }
                    }
                }
                sn = None;
            }
        }
        if let Some(s) = sn {
            dbg!(&s);
            // check start of dig
            if s.start > 0 {
                if let Some(val) = line.get(s.start - 1) {
                    if *val != '.' && !val.is_digit(10) {
                        cnt += s.number;
                        sn = None;
                        println!("add 2 = {val}");
                        continue;
                    }
                }
            }
            // check above
            if s.row > 0 {
                if let Some(above) = matrix.get(s.row - 1) {
                    dbg!(s.get_range(above.len()));
                    if let Some(vals) = above.get(s.get_range(above.len())) {
                        println!("3 - : {:?}", &vals);
                        if let Some(_) = vals.iter().find(|c| !c.is_digit(10) && **c != '.') {
                            cnt += s.number;
                            sn = None;
                            println!("add 3");
                            continue;
                        }
                    }
                }
            }
            // check bottom
            if let Some(below) = matrix.get(s.row + 1) {
                if let Some(vals) = below.get(s.get_range(below.len())) {
                    if let Some(_) = vals.iter().find(|c| !c.is_digit(10) && **c != '.') {
                        cnt += s.number;
                        sn = None;
                        println!("add 4");
                        continue;
                    }
                }
            }
        }
        //check
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_solution_is() {
        let input = include_str!("../../data/test_input1.txt");
        let result = part1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn basic() {
        let input = ".....\n.123.\n.....";
        let result = part1(input);
        assert_eq!(result, 0);
    }
    #[test]
    fn tl() {
        let input = "*....\n.123.\n.....";
        let result = part1(input);
        assert_eq!(result, 123);
    }
    #[test]
    fn tr() {
        let input = "....*\n.123.\n.....";
        let result = part1(input);
        assert_eq!(result, 123);
    }
    #[test]
    fn bl() {
        let input = ".....\n.123.\n*....";
        let result = part1(input);
        assert_eq!(result, 123);
    }
    #[test]
    fn br() {
        let input = ".....\n.123.\n....*";
        let result = part1(input);
        assert_eq!(result, 123);
    }

    #[test]
    fn left_num() {
        let input = "*....\n123..\n.....";
        let result = part1(input);
        assert_eq!(result, 123);
    }
    #[ignore]
    #[test]
    fn right_num() {
        let input = ".....\n..123\n....*";
        let result = part1(input);
        assert_eq!(result, 123);
    }

    #[test]
    fn right_num_not_edge() {
        let input = ".....\n..123\n...*.";
        let result = part1(input);
        assert_eq!(result, 123);
    }

    mod serialnumbers {
        use crate::SerialNumber;

        #[test]
        fn new() {
            let a = SerialNumber::new(1, 2, 3, 2);

            assert_eq!(a.number, 1);
            assert_eq!(a.start, 2);
            assert_eq!(a.end, 3);
            assert_eq!(a.row, 2);
        }
        #[test]
        fn add() {
            let mut a = SerialNumber::new(1, 2, 3, 2);
            let a = a.add(3, 5);
            assert_eq!(a.number, 13);
            assert_eq!(a.start, 2);
            assert_eq!(a.end, 5);
            assert_eq!(a.row, 2);
        }
    }
}
