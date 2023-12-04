use std::collections::HashMap;

/*
The Elf would first like to know which games would have been possible if the bag contained only
12 red cubes, 13 green cubes, and 14 blue cubes?

for each line:
    * split into draws
    * for each draw split into pairs of (number, colour)

    for each pair:
        check if number is greater than the number for that colour in the hashmap:
            if so:
                add game id to sum of game ids
*/

fn get_game_id(line: &str) -> i32 {
    let idx_colon = line.find(":").unwrap();
    let idx_space = line.find(" ").unwrap();

    let id = &line[idx_space + 1..idx_colon];
    let id = id.parse::<i32>().unwrap();

    return id;
}

fn is_impossible(draw: &str) -> bool {
    let cubes: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let pair: Vec<&str> = draw.split_whitespace().collect();

    let digit = pair[0].parse::<i32>().unwrap();
    let colour = pair[1];
    let max_colour = cubes.get(colour).unwrap();

    if digit > *max_colour {
        return true;
    }
    return false;
}

fn process_line(line: &str) -> i32 {
    let id = get_game_id(line);
    let idx_colon = line.find(":").unwrap();
    let line = &line[idx_colon + 2..];
    let draws: Vec<&str> = line.split("; ").collect();

    for draw in draws {
        let pairs: Vec<&str> = draw.split(", ").collect();

        for pair in pairs {
            if is_impossible(pair) {
                return 0;
            }
        }
    }
    return id;
}

fn main() {
    let input = include_str!("input.txt");
    let mut total_game_sum = 0;

    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        total_game_sum += process_line(line)
    }
    println!("{total_game_sum}");
}
