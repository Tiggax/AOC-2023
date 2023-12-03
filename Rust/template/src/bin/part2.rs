
fn main() {
    let input = include_str!("../../data/input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_solution_is() {
        let input = include_str!("../../data/test_input2.txt");
        let result = part2(input);
        assert_eq!(result, "result".to_string());
    }
}
