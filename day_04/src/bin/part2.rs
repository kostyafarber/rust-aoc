use std::collections::HashMap;

/*
Specifically, you win copies of the scratchcards below the winning card equal to the number of matches.
So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.

Copies of scratchcards are scored like normal scratchcards and have the same card number as the card they copied.
 So, if you win a copy of card 10 and it has 5 matching numbers,
 it would then win a copy of the same cards that the
 original card 10 won: cards 11, 12, 13, 14, and 15.
 This process repeats until none of the copies cause you to win any more cards.

 (Cards will never make you copy a card past the end of the table.)

 However many copies you accumulate of a card, when you get to the card, you multiply your winning copies
 by that accumulation
*/

fn remove_identifer(line: &str) -> &str {
    let colon_idx = line.find(":").unwrap();
    return &line[colon_idx + 1..];
}

fn split_numbers(line: &str) -> Vec<&str> {
    return line.split("|").collect::<Vec<&str>>();
}

fn get_numbers(cards: &str) -> Vec<&str> {
    return cards.split(" ").collect();
}

fn remove_empty_strings(cards: Vec<&str>) -> Vec<&str> {
    return cards.into_iter().filter(|x| *x != "").collect();
}

fn get_winning_numbers_in_your_cards<'a>(
    numbers: Vec<&'a str>,
    winning_numbers: Vec<&str>,
) -> Vec<&'a str> {
    return numbers
        .into_iter()
        .filter(|&x| winning_numbers.contains(&x))
        .collect();
}

fn accumulate_map(card_number: usize, winning_numbers: Vec<&str>, map: &mut HashMap<String, i32>) {
    /*
    accumlate
    apply accumulator factor
    */

    map.entry(card_number.to_string()).and_modify(|n| *n += 1);
    let accumlator_factor = *map.get(&card_number.to_string()).unwrap();
    for (i, _win) in winning_numbers.iter().enumerate() {
        let current_card_number = (card_number + i + 1).to_string();
        map.entry(current_card_number)
            .and_modify(|n| *n += 1 * accumlator_factor);
    }
}

fn process_line(line_number: usize, line: &str, map: &mut HashMap<String, i32>) {
    let clean_line = remove_identifer(line);
    let split_numbers = split_numbers(clean_line);

    let winning_line = split_numbers[0];
    let your_line = split_numbers[1];

    let mut winning_numbers = get_numbers(winning_line);
    let mut your_numbers = get_numbers(your_line);

    winning_numbers = remove_empty_strings(winning_numbers);
    your_numbers = remove_empty_strings(your_numbers);

    let wins = get_winning_numbers_in_your_cards(your_numbers, winning_numbers);

    if !wins.is_empty() {
        return accumulate_map(line_number, wins, map);
    }
    map.entry(line_number.to_string()).and_modify(|n| *n += 1);
}

fn create_card_copy_map(number_of_lines: usize) -> HashMap<String, i32> {
    let mut card_copy_count: HashMap<String, i32> = HashMap::new();

    for i in 1..number_of_lines {
        card_copy_count.insert(i.to_string(), 0);
    }

    return card_copy_count;
}

fn process_cards(input: &str, card_copy_map: &mut HashMap<String, i32>) {
    for (line_number, line) in input.lines().enumerate() {
        process_line(line_number + 1, line, card_copy_map);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let num_lines = input.lines().count();
    let mut map = create_card_copy_map(num_lines + 1);
    process_cards(input, &mut map);

    let sum: i32 = map.values().sum();
    println!("sum is: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumlate_map() {
        let tests = vec![
            (
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n \
                 Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
                create_card_copy_map(6),
                HashMap::from_iter(vec![
                    ("1".to_string(), 1),
                    ("2".to_string(), 2),
                    ("3".to_string(), 3),
                    ("4".to_string(), 3),
                    ("5".to_string(), 1),
                ]),
            ),
            (
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n \
                 Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n \
                 Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n \
                 Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n \
                 Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n \
                 Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n",
                create_card_copy_map(7),
                HashMap::from_iter(vec![
                    ("1".to_string(), 1),
                    ("2".to_string(), 2),
                    ("3".to_string(), 4),
                    ("4".to_string(), 8),
                    ("5".to_string(), 14),
                    ("6".to_string(), 1),
                ]),
            ),
        ];

        for (input, card_copy_map, expected) in tests {
            let mut card_copy_map = card_copy_map;
            process_cards(input, &mut card_copy_map);
            assert_eq!(card_copy_map, expected);
        }
    }
}
