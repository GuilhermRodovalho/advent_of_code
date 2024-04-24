use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

struct Instruction {
    direction: char,
    distance: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // should make the two coordinates closer
    // if the two coordinates are already close, do nothing
    fn get_close(&mut self, other: &Coordinate) {
        if self.is_close(other) {
            return;
        }

        if self.x < other.x {
            self.x += 1;
        } else if self.x > other.x {
            self.x -= 1;
        }

        if self.y < other.y {
            self.y += 1;
        } else if self.y > other.y {
            self.y -= 1;
        }
    }

    // should consider the two coordinates close if they are
    // - on the same line and one column apart
    // - on the same column and one line apart
    // - on the same line and same column
    // - on one diagonal apart
    fn is_close(&self, other: &Coordinate) -> bool {
        self.x == other.x && self.y == other.y
            || self.x == other.x && (self.y - other.y).abs() == 1
            || self.y == other.y && (self.x - other.x).abs() == 1
            || (self.x - other.x).abs() == 1 && (self.y - other.y).abs() == 1
    }
}

impl Instruction {
    fn new(direction: char, distance: i32) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

fn main() {
    // Read all the instructions from a file and store them in a vector
    let instructions = read_instructions("input.txt");

    let mut head = Coordinate::new(0, 0);
    let mut tail = Coordinate::new(0, 0);

    // create a set for the coordinates where tail was
    let mut visited: HashSet<Coordinate> = HashSet::new();

    // first problem
    for instruction in &instructions {
        for _ in 0..instruction.distance {
            // add the tail to the set

            match instruction.direction {
                'U' => head.y += 1,
                'D' => head.y -= 1,
                'R' => head.x += 1,
                'L' => head.x -= 1,
                _ => println!("Invalid direction"),
            }
            // after changing the head, should move the tail
            tail.get_close(&head);
            visited.insert(tail);
        }
    }

    println!("Visited {} coordinates", visited.len());

    // second problem

    let mut tail: [Coordinate; 9] = [
        Coordinate::new(0, 0),
        Coordinate::new(0, 0),
        Coordinate::new(0, 0),
        Coordinate::new(0, 0),
        Coordinate::new(0, 0),
        Coordinate::new(0, 0),
        Coordinate::new(0, 0),
        Coordinate::new(0, 0),
        Coordinate::new(0, 0),
    ];
    let mut visited: HashSet<Coordinate> = HashSet::new();

    for instruction in instructions {
        for _ in 0..instruction.distance {
            // add the tail to the set
            visited.insert(tail[tail.len() - 1]);

            match instruction.direction {
                'U' => head.y += 1,
                'D' => head.y -= 1,
                'R' => head.x += 1,
                'L' => head.x -= 1,
                _ => println!("Invalid direction"),
            }
            // after changing the head, should move the tail

            // move the tail
            for (i, mut step) in tail.into_iter().enumerate() {
                if i > 0 {
                    step.get_close(&tail[i - 1]);
                } else {
                    step.get_close(&head);
                }

                tail[i] = step;
            }

            visited.insert(tail[tail.len() - 1]);
        }
    }

    println!("Visited {} coordinates", visited.len());
}

// read all the instructions from a file and store them in a vector
// the format is "R 32" or "L 12"
fn read_instructions(filename: &str) -> Vec<Instruction> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    let mut instructions = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let infos: Vec<&str> = line.trim().split_whitespace().collect();

        let direction = infos[0].chars().next().expect("Could not read direction");
        let distance = infos[1].parse::<i32>().expect("Could not read distance");

        instructions.push(Instruction::new(direction, distance));
    }

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_is_close() {
        let coordinate = Coordinate::new(0, 0);
        let mut other = Coordinate::new(0, 1);

        assert!(coordinate.is_close(&other));

        other.y = 2;
        assert!(!coordinate.is_close(&other));

        other.y = 0;
        other.x = 1;
        assert!(coordinate.is_close(&other));

        other.y = 1;
        other.x = 1;
        assert!(coordinate.is_close(&other));

        other.x = 2;
        other.y = 0;
        assert!(!coordinate.is_close(&other));

        other.x = -1;
        other.y = 0;
        assert!(coordinate.is_close(&other));

        other.x = -1;
        other.y = -1;
        assert!(coordinate.is_close(&other));
    }

    #[test]
    fn test_coordinate_get_close() {
        let mut coordinate = Coordinate::new(0, 0);
        let other = Coordinate::new(0, 1);

        coordinate.get_close(&other);
        assert_eq!(coordinate, Coordinate::new(0, 0));

        coordinate.get_close(&other);
        assert_eq!(coordinate, Coordinate::new(0, 0));

        coordinate.get_close(&Coordinate::new(1, 1));
        assert_eq!(coordinate, Coordinate::new(0, 0));

        coordinate.get_close(&Coordinate::new(2, 1));
        assert_eq!(coordinate, Coordinate::new(1, 1));

        coordinate.get_close(&Coordinate::new(0, 0));
        assert_eq!(coordinate, Coordinate::new(1, 1));

        coordinate.get_close(&Coordinate::new(1, 3));
        assert_eq!(coordinate, Coordinate::new(1, 2));
    }
}
