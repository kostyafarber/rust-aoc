/*
each card has two lists of numbers separated by a vertical bar (|):
you have to figure out which of the numbers you have appear in the list of winning numbers.
The first match makes the card worth one point and each match after the first doubles the point
value of that card.

can the cards repeat in either deck and what are the implications?
    - if in winning cards, doesn't really make any sense if they repeat
    - in your cards it does, because you can increase your score again.
    - does matching a winning number consume it?
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

fn get_score_for_winning_numbers(wins: Vec<&str>) -> i32 {
    return wins[..wins.len() - 1].iter().fold(1, |acc, _x| acc * 2);
}

fn process_line(line: &str) -> i32 {
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
        return get_score_for_winning_numbers(wins);
    }
    return 0;
}

fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let score = lines.iter().fold(0, |acc, x| acc + process_line(x));
    println!("score is: {score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_number_of_winning_cards() {
        let card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let clean_line = remove_identifer(card);
        let split_numbers = split_numbers(clean_line);

        let winning_line = split_numbers[0];
        let your_line = split_numbers[1];

        let mut winning_numbers = get_numbers(winning_line);
        let mut your_numbers = get_numbers(your_line);

        winning_numbers = remove_empty_strings(winning_numbers);
        your_numbers = remove_empty_strings(your_numbers);

        let wins = get_winning_numbers_in_your_cards(your_numbers, winning_numbers);

        assert_eq!(wins.len(), 4);
    }

    #[test]
    fn correct_score_on_card() {
        let card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let score = process_line(card);

        assert_eq!(score, 8);
    }

    #[test]
    fn it_works() {
        let scratch_cards = " \
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n \
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n \
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n \
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n \
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n \
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n";

        let lines: Vec<&str> = scratch_cards.lines().collect();

        let score = lines.iter().fold(0, |acc, x| acc + process_line(x));
        assert_eq!(score, 13);
    }
}
