/*
create a 2D grid out of the input then:

need a way to store digits and then parse into a number

iterate over all the chars:
    if digit:
        check up, left, down, right
        check nw, ne, sw, se

        if non_alphanumerical symbol is reachable from checks:
            add that number to the cum sum

*/

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
        if x <= bounds.left || x >= bounds.right || y < bounds.up || y >= bounds.down {
            return false;
        }
        return true;
    }

    pub fn check_adjacent(&self, x: i32, y: i32) -> bool {
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

        for (dx, dy) in pairs {
            let new_x = x + dx;
            let new_y = y + dy;

            if self.in_bounds(new_x, new_y) {
                let pos = self.grid[new_x as usize][new_y as usize];
                if !pos.is_digit(10) && pos != '.' {
                    return true;
                }
            }
        }
        return false;
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

    let mut cum_sum = 0;
    let mut number = 0;
    let mut add = false;
    let mut in_digit = false;

    for (i, line) in puzzle.grid.iter().enumerate() {
        println!("line {i}");
        println!();
        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                in_digit = true;
                number = (number * 10) + c.to_digit(10).unwrap();
                if puzzle.check_adjacent(i as i32, j as i32) {
                    add = true;
                }
            }

            if !c.is_digit(10) || j == puzzle.grid.len() - 1 {
                if add {
                    cum_sum += number;
                    add = false;
                }
                if in_digit {
                    in_digit = false;
                    number = 0;
                }
            }
        }
        number = 0;
        println!();
    }
    println!("Sum of parts are: {cum_sum}");
}
