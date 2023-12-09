struct Bounds {
    left: i32,
    right: i32,
    up: i32,
    down: i32,
}
struct Puzzle {
    grid: Vec<Vec<char>>,
}

impl Puzzle {
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        let bounds = Bounds {
            left: 0,
            right: (self.grid[0].len()) as i32,
            up: 0,
            down: (self.grid.len()) as i32,
        };
        if x < bounds.left || x >= bounds.right || y < bounds.up || y >= bounds.down {
            return false;
        }
        return true;
    }

    pub fn check_adjacent(&self, x: i32, y: i32) -> Vec<i32> {
        let pairs: Vec<(i32, i32)> = vec![
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            (-1, 1),
            (1, -1),
            (-1, -1),
            (1, 1),
        ];

        let mut numbers: Vec<i32> = Vec::new();
        for (dx, dy) in pairs {
            let new_x = x + dx;
            let new_y = y + dy;

            if self.in_bounds(new_x, new_y) {
                let pos = self.grid[new_y as usize][new_x as usize];
                if pos.is_digit(10) {
                    let line = &self.grid[new_y as usize];
                    let number = self.get_number(line, x as usize);
                    numbers.push(number);
                }
            }
        }
        return numbers;
    }

    fn parse_number(&self, digits: Vec<char>) -> i32 {
        let number: String = digits.iter().collect();
        return number.parse::<i32>().unwrap();
    }

    fn get_number(&self, line: &Vec<char>, x: usize) -> i32 {
        let left = &line[0..x + 1];
        let right = &line[x..line.len()];

        let find_start = left.iter().rev().position(|x| !x.is_digit(10));
        let find_end = right.iter().position(|x| !x.is_digit(10));

        let start = if let Some(i) = find_start {
            x - i + 1
        } else {
            0
        };
        let end = if let Some(i) = find_end {
            x + i
        } else {
            line.len() - 1
        };

        if start == end {
            return self.parse_number(vec![line[start]]);
        }
        return self.parse_number(line[start..end].to_vec());
    }
}

fn make_grid(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<Vec<char>> = input
        .lines()
        .into_iter()
        .map(|c| c.chars().collect())
        .collect();

    return lines;
}

fn main() {
    let input = include_str!("input.txt");

    let puzzle = Puzzle {
        grid: make_grid(input),
    };
}

#[cfg(test)]

mod tests {
    use core::num;

    use super::*;

    #[test]
    fn number_matches() {
        let tests: Vec<(Vec<char>, usize, i32, &str)> = vec![
            (
                "......545..........".chars().collect(),
                7,
                545,
                "three digit number with index in the middle",
            ),
            (
                "45.................".chars().collect(),
                1,
                45,
                "two digits number at the start of line",
            ),
            (
                "..................7".chars().collect(),
                18,
                7,
                "one digit number at the end of the line",
            ),
            (
                ".......6..........".chars().collect(),
                7,
                6,
                "one digit in the middle of the line",
            ),
            (
                ".............678...".chars().collect(),
                13,
                678,
                "three digit number with index at the start of the number",
            ),
        ];

        let puzzle = Puzzle {
            grid: vec![vec!['c']],
        };
        for (line, idx, expected, desc) in tests {
            assert_eq!(puzzle.get_number(&line, idx), expected, "{}", desc);
        }
    }

    #[test]
    fn check_adjacent() {
        let input = ".................58...................\n\
                           ..................*...................\n\
                           ...................102................\n";

        let puzzle = Puzzle {
            grid: make_grid(input),
        };

        let numbers = puzzle.check_adjacent(18, 1);
        let expected = vec![58, 102];

        assert_eq!(expected, numbers);
    }
}
