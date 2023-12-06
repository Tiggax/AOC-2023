fn main() {
    let input = include_str!("../../data/input2.txt");
    let output = part2(input);
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
        res
    }
}

fn part2(input: &str) -> usize {
    let input: Vec<usize> = input.lines().map(|card| Card::new(card).points()).collect();

    let mut cards = vec![1; input.len()];

    for (i, point) in input.iter().enumerate() {
        dbg!(&cards);
        println!("at {i} with {point} points");
        for p in i + 1..(i + 1 + point) {
            println!("adding at {p}");
            cards[p] += cards[i]
        }
    }

    cards.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_solution_is() {
        let input = include_str!("../../data/test_input2.txt");
        let result = part2(input);
        assert_eq!(result, 30);
    }
}
