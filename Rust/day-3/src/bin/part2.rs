fn main() {
    let input = include_str!("../../data/input2.txt");
    let output = part2(input);
    dbg!(output);
    assert_eq!(output, 77509019);
}

fn part2(input: &str) -> u64 {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut cnt = 0;
    for (lid, line) in matrix.iter().enumerate() {
        for (cid, char) in line.iter().enumerate() {
            println!("{lid}-{cid}:{char}");
            if *char == '*' {
                println!("found");
                let mut is_seperate = true;
                let mut numbers = Vec::new();
                'rows: for x in -1..=1 {
                    for y in vec![0, 1, -1] {
                        let r = lid as i32 + x;
                        let c = cid as i32 + y;
                        if r < 0 || c < 0 || r > matrix.len() as i32 || c > matrix[0].len() as i32 {
                            continue;
                        }
                        println!("inner:{r}-{c}");
                        if y == 0 {
                            if matrix[r as usize][c as usize].is_digit(10) {
                                is_seperate = false;
                                println!("its not seperate");
                            }
                            println!("check middle:{is_seperate}");
                        }
                        println!("check:{}-{}: {}", r, c, matrix[r as usize][c as usize]);
                        if let Some(nm) = matrix[r as usize][c as usize].to_digit(10) {
                            println!("found: {nm}");
                            let mut out = nm as u64;
                            if is_seperate {
                                let number = match y {
                                    -1 => {
                                        if c < 0 {
                                            continue;
                                        }
                                        let mut p = (c - 1) as usize;
                                        let mut e = 1;
                                        'inner: while p >= 0 {
                                            if let Some(val) = matrix[r as usize][p].to_digit(10) {
                                                println!("add:{val} to {out}");
                                                out += val as u64 * 10u64.pow(e);
                                                e += 1;
                                                if p == 0 {
                                                    break 'inner;
                                                }
                                                p -= 1;
                                            } else {
                                                break 'inner;
                                            }
                                        }
                                        out
                                    }
                                    1 => {
                                        let mut p = (c + 1) as usize;
                                        while let Some(chr) = matrix[r as usize].get(p) {
                                            if let Some(val) = chr.to_digit(10) {
                                                out *= 10;
                                                out += val as u64;
                                                p += 1;
                                            } else {
                                                break;
                                            }
                                        }
                                        out
                                    }
                                    x => panic!("IDK wtf happend here? how did '{x}' get here?"),
                                };
                                println!("found: {:?}", number);
                                numbers.push(number);
                            } else {
                                println!("in else");
                                // skupna row stevilka
                                // parse stevika
                                // throw and pull
                                let mut e = 1;
                                let mut p = (c + 1) as usize;
                                while let Some(val) = matrix[r as usize][p].to_digit(10) {
                                    println!("in second: at {r}-{p}: {val}");
                                    out *= 10;
                                    out += val as u64;
                                    e += 1;
                                    p += 1;
                                }

                                if c < 0 {
                                    continue;
                                }
                                let mut p = (c - 1) as usize;
                                'inner: while p >= 0 {
                                    if let Some(val) = matrix[r as usize][p].to_digit(10) {
                                        println!("add:{val} to {out}");
                                        out += val as u64 * 10u64.pow(e);
                                        e += 1;
                                        if p == 0 {
                                            break 'inner;
                                        }
                                        p -= 1;
                                    } else {
                                        break 'inner;
                                    }
                                }

                                numbers.push(out);
                                continue 'rows;
                            }
                        }
                    }
                }
                // after looking around mamo vektor stevil
                println!("at {}-{}: found {:?}", lid, cid, &numbers);
                cnt += if numbers.len() == 2 {
                    numbers.iter().product::<u64>()
                } else {
                    0u64
                };
                println!("{cnt}");
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
        let input = include_str!("../../data/test_input2.txt");
        let result = part2(input);
        assert_eq!(result, 467835);
    }
}
