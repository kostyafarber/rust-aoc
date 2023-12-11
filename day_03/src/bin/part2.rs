use std::collections::HashSet;

struct Bounds {
    left: i32,
    right: i32,
    up: i32,
    down: i32,
}
struct Puzzle {
    grid: Vec<Vec<char>>,
    seen_positions: HashSet<(usize, usize)>,
}

impl Puzzle {
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        let bounds = Bounds {
            left: 0,
            right: (self.grid[0].len()) as i32,
            up: 0,
            down: (self.grid.len()) as i32,
        };
        if x < bounds.left
            || x >= bounds.right
            || y < bounds.up
            || y >= bounds.down
            || self.seen_positions.contains(&(x as usize, y as usize))
        {
            return false;
        }
        return true;
    }

    pub fn check_adjacent(&mut self, x: i32, y: i32) -> Vec<i32> {
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
                    let number = self.get_number(new_y as usize, new_x as usize);
                    numbers.push(number);
                }
            }
        }
        self.seen_positions.clear();
        return numbers;
    }

    fn parse_number(&self, digits: Vec<char>) -> i32 {
        let number: String = digits.iter().collect();
        return number.parse::<i32>().unwrap();
    }

    fn get_number(&mut self, y: usize, x: usize) -> i32 {
        let line = &self.grid[y];
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

        for x in start..end - 1 {
            self.seen_positions.insert((x, y));
        }

        if start == end {
            return self.parse_number(vec![line[start]]);
        }
        return self.parse_number(line[start..end].to_vec());
    }

    fn get_gear_ratio(&self, numbers: Vec<i32>) -> i32 {
        return numbers[0] * numbers[1];
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

    let mut puzzle = Puzzle {
        grid: make_grid(input),
        seen_positions: HashSet::new(),
    };

    let mut sum_gear_ratios = 0;
    for y in 0..puzzle.grid.len() {
        for x in 0..puzzle.grid[y].len() {
            let pos = puzzle.grid[y][x];
            if pos == '*' {
                let numbers = puzzle.check_adjacent(x as i32, y as i32);
                if numbers.len() == 2 {
                    let gear_ratio = puzzle.get_gear_ratio(numbers);
                    sum_gear_ratios += gear_ratio;
                }
            }
        }
    }
    println!("Sum of gear ratios: {}", sum_gear_ratios);
}

#[cfg(test)]

mod tests {
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

        for (line, idx, expected, desc) in tests {
            let mut puzzle = Puzzle {
                grid: vec![],
                seen_positions: HashSet::new(),
            };
            puzzle.grid.push(line);
            assert_eq!(puzzle.get_number(0, idx), expected, "{}", desc);
        }
    }

    #[test]
    fn check_adjacent() {
        let input = ".................58...................\n\
                           ..................*...................\n\
                           ...................102................\n";

        let mut puzzle = Puzzle {
            grid: make_grid(input),
            seen_positions: HashSet::new(),
        };

        let numbers = puzzle.check_adjacent(18, 1);
        let expected = vec![58, 102];

        assert_eq!(expected, numbers);
    }

    fn check_solve_test_puzzle() {
        let input = "
            467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..\n";

        let mut puzzle = Puzzle {
            grid: make_grid(input),
            seen_positions: HashSet::new(),
        };

        let mut sum_gear_ratios = 0;
        for y in 0..puzzle.grid.len() {
            for x in 0..puzzle.grid[y].len() {
                let pos = puzzle.grid[y][x];
                if pos == '*' {
                    let numbers = puzzle.check_adjacent(x as i32, y as i32);
                    let gear_ratio = puzzle.get_gear_ratio(numbers);
                    sum_gear_ratios += gear_ratio;
                }
            }
        }
        assert_eq!(sum_gear_ratios, 467835);
    }
}
