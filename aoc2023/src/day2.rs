use std::collections::HashMap;

fn get_colors_values_from_take(take: &str) -> [i32; 3] {
    let mut colors_values = [0, 0, 0];
    let _ = take
        .split(',')
        .map(|pair| {
            let pair: Vec<&str> = pair.split_whitespace().collect();

            match *pair.get(1).unwrap() {
                "red" => {
                    colors_values[0] = pair[0].parse().unwrap();
                }
                "green" => {
                    colors_values[1] = pair[0].parse().unwrap();
                }
                "blue" => {
                    colors_values[2] = pair[0].parse().unwrap();
                }
                _ => {}
            }
        })
        .collect::<()>();

    colors_values
}

fn get_largests_colors(line: &str) -> [i32; 3] {
    let mut largest_colors = [0, 0, 0];
    // remove the "Game N" part
    let binding = line.split(':').collect::<Vec<&str>>();
    let new_line = binding.get(1).unwrap();

    let _ = new_line
        .split(';')
        .map(|take| {
            let take_values = get_colors_values_from_take(take);
            for (i, v) in take_values.iter().enumerate() {
                if &largest_colors[i] < v {
                    largest_colors[i] = *v
                }
            }
        })
        .collect::<()>();

    largest_colors
}

fn calculate_power(values: [i32; 3]) -> i32 {
    values[0] * values[1] * values[2]
}

fn is_valid_take(take: &str) -> bool {
    let valid_takes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let parts: Vec<&str> = take.split_whitespace().collect();
    let quantity = parts.first().unwrap().parse::<i32>().unwrap();
    let color = parts.get(1).unwrap();

    valid_takes.get(color).unwrap() >= &quantity
}

fn solve_game_take(take: &str) -> bool {
    take.split(',').all(is_valid_take)
}

fn solve_game_line(line: &str) -> i32 {
    let game_value = line
        .split(':')
        .next()
        .unwrap()
        .split(' ')
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let games = line
        .split(':')
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .trim();

    let is_valid_game = games.split(';').map(solve_game_take).all(|x| x);

    match is_valid_game {
        true => game_value,
        false => 0,
    }
}

pub fn solve_puzzle1(input: &str) -> i32 {
    input.lines().map(solve_game_line).sum()
}

pub fn solve_puzzle2(input: &str) -> i32 {
    input
        .lines()
        .map(get_largests_colors)
        .fold(0, |acc, f| acc + calculate_power(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(solve_game_line(input), 1);

        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        // should return 0 because the game is invalid (20 red)
        assert_eq!(solve_game_line(input), 0);

        let input = "Game 23: 8 green, 6 blue, 9 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(solve_game_line(input), 23);
    }

    #[test]
    fn test_is_valid_take() {
        let take = "4 blue";

        assert!(is_valid_take(take));
        assert_eq!(is_valid_take("24 green"), false);
    }

    #[test]
    fn test_solve_puzzle1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_puzzle1(input), 8)
    }

    #[test]
    fn test_get_largest_colors() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(get_largests_colors(input), [4, 2, 6]);
    }

    #[test]
    fn test_get_colors_values_from_take() {
        assert_eq!(get_colors_values_from_take("3 blue, 4 red"), [4, 0, 3]);
        assert_eq!(get_colors_values_from_take("1 red, 2 green"), [1, 2, 0]);
    }

    #[test]
    fn test_solve_puzzle2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_puzzle2(input), 2286)
    }
}
