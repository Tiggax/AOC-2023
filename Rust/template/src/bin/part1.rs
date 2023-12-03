fn main() {
    let input = include_str!("../../data/input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_solution_is() {
        let input = include_str!("../../data/test_input1.txt");
        let result = part1(input);
        assert_eq!(result, "result".to_string());
    }
}
