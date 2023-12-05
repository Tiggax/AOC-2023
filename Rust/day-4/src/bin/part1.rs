fn main() {
    let input = include_str!("../../data/input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Card {
    my: Vec<u32>,
    winning: Vec<u32>,
}

impl Card {
    fn new(card: &str) -> Self {
        let line = card.split(':').last();
        let res: Vec<Vec<u32>> = line
            .expect("There was weird input")
            .trim()
            .split('|')
            .map(|vec| {
                vec.trim()
                    .split_whitespace()
                    .map(|n| n.trim().parse().unwrap())
                    .collect()
            })
            .collect();
        Card {
            winning: res[0].clone(),
            my: res[1].clone(),
        }
    }

    fn points(self) -> usize {
        let (my, win) = (self.my, self.winning);

        let res = my
            .iter()
            .map(|v| win.contains(v))
            .filter(|f| *f)
            .collect::<Vec<bool>>()
            .len();

        match res {
            0 => 0,
            x => 1 << (x - 1),
        }
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|card| Card::new(card).points())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_solution_is() {
        let input = include_str!("../../data/test_input1.txt");
        let result = part1(input);
        assert_eq!(result, 13);
    }
}
