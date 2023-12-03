fn main() {
    let input = include_str!("../../data/input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut cnt = 0;
    for word in lines {
        let vec: Vec<u32> = word.chars().filter_map(|c| c.to_digit(10)).collect();
        let len = vec.len();
        let (first, last) = (vec[0], vec[len - 1]);

        cnt += (first * 10) + last;
    }

    cnt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_solution_is() {
        let input = include_str!("../../data/test_input1.txt");
        let result = part1(input);
        assert_eq!(result, "142".to_string());
    }
}
