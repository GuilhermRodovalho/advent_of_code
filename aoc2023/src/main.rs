fn create_number_map() -> Vec<(&'static str, u32)> {
    vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
}

enum Position {
    First,
    Last,
}

fn get_number(line: &str, position: Position) -> u32 {
    let numbers = create_number_map();
    let iter: Box<dyn Iterator<Item = (usize, char)>> = match Position::First {
        Position::First => Box::new(line.char_indices()),
        Position::Last => Box::new(line.char_indices().rev()),
    };

    for (i, c) in iter {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
        for (n, v) in &numbers {
            match position {
                Position::First if i + n.len() < line.len() && line[i..i + n.len()].contains(n) => {
                    return *v;
                }
                Position::Last if line[i..].contains(n) => {
                    return *v;
                }
                _ => continue,
            }
        }
    }

    unreachable!()
}

fn solve_puzze(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let first_number = get_number(line, Position::First).to_string();
            let last_number = get_number(line, Position::Last).to_string();
            format!("{}{}", first_number, last_number).parse::<i32>().unwrap()
        })
        .sum()
}

fn main() {
    // read all content from input.txt
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let res = solve_puzze(&input);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        assert_eq!(solve_puzze(input), 142)
    }

    #[test]
    fn second_case() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solve_puzze(input), 281)
    }

    #[test]
    fn test_get_first_number() {
        assert_eq!(3, get_first_number("threeightfour"));
        assert_eq!(1, get_first_number("1eighthree"));
        assert_eq!(2, get_first_number("two1nine"));
    }
}
