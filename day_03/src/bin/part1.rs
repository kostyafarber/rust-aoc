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

fn check_adjacent() {}

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
    let grid = make_grid(input);

    for line in grid {}
}
