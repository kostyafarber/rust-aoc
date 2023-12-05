/*
The minimum number of cubes to make a game possible will be:
    for each line:
        record the max number for each colour seen.

        for each set of max numbers for each colour on each line:
            multiply them by each other
            add them add to a cum sum
*/

use std::collections::HashMap;

fn process_line(line: &str) -> i32 {
    let idx_colon = line.find(":").unwrap();
    let line = &line[idx_colon + 2..];
    let draws: Vec<&str> = line.split("; ").collect();

    let mut min_possible_colours: HashMap<&str, i32> =
        HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    for draw in draws {
        let pairs: Vec<&str> = draw.split(", ").collect();

        for pair in pairs {
            let number_colour_pair: Vec<&str> = pair.split_whitespace().collect();
            let number = number_colour_pair[0].parse::<i32>().unwrap();
            let colour = number_colour_pair[1];
            let max_colour = min_possible_colours.get(colour).unwrap();

            if number > *max_colour {
                *min_possible_colours.entry(colour).or_insert(0) = number;
            }
        }
    }

    let mut power_sum = 1;
    for (_, k) in min_possible_colours {
        power_sum *= k;
    }
    return power_sum;
}

fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut cum_power_sum = 0;
    for line in lines {
        cum_power_sum += process_line(line);
    }
    println!("The power sum is: {cum_power_sum}")
}
