/*

Traverse each line and:
    if it's a non alphanumerical character then:
        for each of the digits:
            take the length of the digit

            check if the word from the current index + the length of the current word in digits
            is the digit

            if so:
                make this the current digit, using the map
*/

use std::{cmp::max, collections::HashMap};

enum FoundWord {
    Found(u32),
    NotFound,
}

fn process_character(line: &str, i: usize) -> FoundWord {
    let digit_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut found_word: &str = "";
    for (&word, &digit) in &digit_map {
        let len_word = word.len();
        if (i + len_word) <= line.len() {
            let curr_word = &line[i..i + len_word];
            if word == curr_word {
                found_word = word;
            }
        }
    }

    if found_word != "" {
        return FoundWord::Found(*digit_map.get(found_word).unwrap());
    } else {
        return FoundWord::NotFound;
    }
}

fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.split("\n").collect();

    let mut first_digit = 0;
    let mut second_digit = 0;

    let mut calibration_sum = 0;
    for line in lines {
        for (i, c) in line.char_indices() {
            if c.is_digit(10) {
                if first_digit == 0 {
                    first_digit = c.to_digit(10).unwrap();
                } else {
                    second_digit = c.to_digit(10).unwrap();
                }
            }

            if c.is_alphabetic() {
                let found_word = process_character(line, i);
                match found_word {
                    FoundWord::Found(word) => {
                        if first_digit == 0 {
                            first_digit = word
                        } else {
                            second_digit = word
                        }
                    }

                    FoundWord::NotFound => (),
                }
            }
        }

        println!("first digit is: {first_digit} and second digit is: {second_digit}");
        if second_digit == 0 {
            let single_digit = max(first_digit, second_digit);
            let number = (single_digit * 10) + single_digit;
            println!("Number is: {number}");
            calibration_sum += number;
        } else {
            let number = (first_digit * 10) + second_digit;
            println!("Sum is: {number}");
            calibration_sum += number;
        }
        first_digit = 0;
        second_digit = 0;
    }
    println!("Sum is {calibration_sum}");
}
