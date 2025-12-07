pub fn part1(contents: String) -> String {
    let mut math_grid = Vec::new();
    for line in contents.lines() {
        let mut math_line = Vec::new();
        for val in line.split_ascii_whitespace() {
            math_line.push(val);
        }
        math_grid.push(math_line);
    }

    let mut grand_total = 0;

    for x in 0..math_grid[0].len() {
        let mut numbers: Vec<u64> = Vec::new();
        for y in 0..math_grid.len() - 1 {
            numbers.push(math_grid[y][x].parse().unwrap());
        }
        let sum: u64 = match math_grid[math_grid.len() - 1][x] {
            "*" => numbers.into_iter().product(),
            "+" => numbers.into_iter().sum(),
            _ => {
                panic!("Unknown")
            }
        };
        grand_total += sum;
    }

    grand_total.to_string()
}

enum Operator {
    Sum,
    Multiply,
}

pub fn part2(contents: String) -> String {
    let items = Vec::new();
    let mut math_grid = items;
    for line in contents.lines() {
        let mut math_line = Vec::new();
        for val in line.chars() {
            math_line.push(val);
        }
        math_grid.push(math_line);
    }

    let mut grand_total: u64 = 0;

    let mut operator = Operator::Sum;
    let mut numbers: Vec<u64> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    for x in 0..math_grid[0].len() {
        let mut whitespace = 0;
        for y in 0..math_grid.len() {
            let element = math_grid[y][x];
            if y == math_grid.len() - 1 && !stack.is_empty() {
                let num: u64 = stack.iter().cloned().collect::<String>().parse().unwrap();
                numbers.push(num);
                stack.clear();
            }
            match element {
                '*' => {
                    operator = Operator::Multiply;
                }
                '+' => {
                    operator = Operator::Sum;
                }
                ' ' => {
                    whitespace += 1;
                    if whitespace == math_grid.len() {
                        grand_total += do_math(&operator, &mut numbers);
                    }
                }
                num => {
                    stack.push(num);
                }
            }
        }
    }

    grand_total += do_math(&operator, &mut numbers);

    grand_total.to_string()
}

fn do_math(operator: &Operator, numbers: &mut Vec<u64>) -> u64 {
    let total: u64 = match operator {
        Operator::Multiply => numbers.iter().product(),
        Operator::Sum => numbers.iter().sum(),
    };
    numbers.clear();
    total
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/6/real.txt").unwrap()),
            "6725216329103"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/6/real.txt").unwrap()),
            "10600728112865"
        );
    }
}
