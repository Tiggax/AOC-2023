
fn main() {
    let input = include_str!("../../data/input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines = input.lines();
    let mut cnt = 0;
    for line in lines {
        let mut iter = (0..line.len()).filter_map(|f| { // I helped myself with Chris Biscardis solution: https://youtu.be/JOgQMjpGum0
            let line_slice = &line[f..];
            let res = if line_slice.starts_with("one") {
                '1'
            } else if line_slice.starts_with("two") {
                '2'
            } else if line_slice.starts_with("three") {
                '3'
            } else if line_slice.starts_with("four") {
                '4'
            } else if line_slice.starts_with("five") {
                '5'
            } else if line_slice.starts_with("six") {
                '6'
            } else if line_slice.starts_with("seven") {
                '7'
            } else if line_slice.starts_with("eight") {
                '8'
            } else if line_slice.starts_with("nine") {
                '9'
            } else {
                line_slice.chars().next().unwrap()
            };

            res.to_digit(10)
        });
        let f = iter.next().expect("wtf no num?");
        let out = match iter.last() {
            Some(n) => f * 10 + n,
            None => f * 10 + f,
        };
        cnt += out;
    }
    cnt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_solution_is() {
        let input = include_str!("../../data/test_input2.txt");
        let result = part2(input);
        assert_eq!(result, "281".to_string());
    }
}
