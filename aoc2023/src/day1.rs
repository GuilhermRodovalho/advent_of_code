// Função para criar e retornar o HashMap desejado
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

// Variável estática que armazena o HashMap retornado pela função
fn get_first_number(line: &str) -> u32 {
    let numbers = create_number_map();

    for (i, c) in line.char_indices() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
        for (n, v) in &numbers {
            if i + n.len() < line.len() && line[i..i + n.len()].contains(n) {
                return *v;
            }
        }
    }

    unreachable!()
}

fn get_last_number(line: &str) -> u32 {
    let numbers = create_number_map();

    for (i, c) in line.char_indices().rev() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
        for (n, v) in &numbers {
            if line[i..].contains(n) {
                return *v;
            }
        }
    }

    unreachable!()
}

pub fn solve_puzze(input: &str) -> i32 {
    input
        .lines()
        .map(|linha| {
            let mut numero = String::new();

            // acha o primeiro número
            numero.push_str(get_first_number(linha).to_string().as_str());

            // acha o último número
            numero.push_str(get_last_number(linha).to_string().as_str());

            numero
        })
        .fold(0, |acc, f| acc + f.parse::<i32>().unwrap())
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

    // #[test]
    // fn test_parse_str() {
    //     let line = "two1nine";
    //     assert_eq!("219", parse_str(&line));

    //     assert_eq!("1ight", parse_str("oneight"));
    //     assert_eq!("8wo3", parse_str("eightwothree"));
    // }
}
