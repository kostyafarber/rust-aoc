use std::collections::HashMap;

/*
The Elf would first like to know which games would have been possible if the bag contained only
12 red cubes, 13 green cubes, and 14 blue cubes?
*/

fn main() {
    let cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let input = include_str!("input.txt");

    let lines: Vec<&str> = input.lines().collect();

    let line = lines[0];
    let idx = line.find(":").unwrap();
    let line = &line[idx + 2..];

    let draws: Vec<&str> = line.split("; ").collect();
    let draw: Vec<&str> = draws[0].split(", ").collect();
    let pair: Vec<&str> = draw[0].split_whitespace().collect();

    println!("{:?}", line);
    println!("{:?}", draws);
    println!("{:?}", draw);
    println!("{:?}", pair);

    let digit = pair[0].parse::<i32>().unwrap();
    let colour = pair[1];

    let max_colour = cubes.get(colour).unwrap();

    println!("digit is: {:?}", digit);
    println!("colour is: {colour}",);
    println!("max colour is: {:?}", max_colour);
}
