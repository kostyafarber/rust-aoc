/*
starting from the first line of input:
    take the difference recursively until the current row contains all zeroes

can you extrapolate for each history successfully?

*/

fn extract_numbers(line: &str) -> Vec<i32> {
    let numbers = line
        .split_ascii_whitespace()
        .into_iter()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    return numbers;
}

fn get_difference(numbers: &Vec<i32>) -> Vec<i32> {
    let mut differences: Vec<i32> = vec![];

    for i in 0..numbers.len() - 1 {
        let difference = numbers[i + 1] - numbers[i];
        differences.push(difference)
    }
    return differences;
}

fn check_for_all_zeroes(numbers: &Vec<i32>) -> bool {
    let check: Vec<&i32> = numbers.into_iter().filter(|&&n| n == 0).collect();
    if check.len() == numbers.len() {
        return true;
    }
    return false;
}

fn produce_history_vector(inital_history: Vec<i32>) -> Vec<Vec<i32>> {
    let mut expand_history: Vec<Vec<i32>> = vec![inital_history];

    loop {
        let difference = get_difference(expand_history.last().unwrap());
        if check_for_all_zeroes(&difference) || difference.is_empty() {
            expand_history.push(difference);
            break;
        }
        expand_history.push(difference);
    }
    return expand_history;
}

fn fill_history_vector(history_vector: &mut Vec<Vec<i32>>) {
    /*
    the first (last) added value will always be zero, and the
    second (second last) value added will always be the same as we are always
    adding zero that number, hence the first block before the for loop
    */

    let vec_length = history_vector.len() - 1;
    let second_last_vec_length = history_vector[vec_length - 1].len() - 1;

    history_vector[vec_length].push(0);

    let second_last_value = history_vector[vec_length - 1][second_last_vec_length];
    history_vector[vec_length - 1].push(second_last_value);

    // discard the zero array
    // and the first (last) to avoid out of bounds error
    for i in (1..vec_length).rev() {
        // let sequence_to_add_to_len = history_vector[i - 1].len() - 1;
        // let sequence_to_add_len = history_vector[i].len() - 1;
        let value_to_add_to = history_vector[i - 1][0];
        let value_to_add = history_vector[i][0];

        history_vector[i - 1].insert(0, (-1 * value_to_add) + value_to_add_to);
    }
}

fn get_prediction(sequence: &Vec<i32>) -> i32 {
    return sequence[0];
}

fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for line in lines {
        let numbers = extract_numbers(line);
        let mut initial_history = produce_history_vector(numbers);
        fill_history_vector(&mut initial_history);
        let prediction = &initial_history[0];

        sum += get_prediction(prediction);
    }

    println!("sum is: {}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_difference() {
        let tests = vec![
            (extract_numbers("0 3 6 9 12 15"), vec![3, 3, 3, 3, 3]),
            (extract_numbers("3 3 3 3 3"), vec![0, 0, 0, 0]),
        ];

        for (numbers, expected) in tests {
            assert_eq!(get_difference(&numbers), expected)
        }
    }

    #[test]
    fn test_produce_history_vector() {
        let tests = vec![
            (
                extract_numbers("0 3 6 9 12 15"),
                vec![
                    vec![0, 3, 6, 9, 12, 15],
                    vec![3, 3, 3, 3, 3],
                    vec![0, 0, 0, 0],
                ],
            ),
            (
                extract_numbers("1 3 6 10 15 21"),
                vec![
                    vec![1, 3, 6, 10, 15, 21],
                    vec![2, 3, 4, 5, 6],
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0],
                ],
            ),
            (
                extract_numbers("10 13 16 21 30 45"),
                vec![
                    vec![10, 13, 16, 21, 30, 45],
                    vec![3, 3, 5, 9, 15],
                    vec![0, 2, 4, 6],
                    vec![2, 2, 2],
                    vec![0, 0],
                ],
            ),
        ];

        for (initial_history, expected_history) in tests {
            assert_eq!(produce_history_vector(initial_history), expected_history);
        }
    }

    #[test]
    fn test_fill_history_vector() {
        let tests = vec![
            (
                produce_history_vector(extract_numbers("10 13 16 21 30 45")),
                vec![
                    vec![5, 10, 13, 16, 21, 30, 45],
                    vec![5, 3, 3, 5, 9, 15],
                    vec![-2, 0, 2, 4, 6],
                    vec![2, 2, 2, 2],
                    vec![0, 0, 0],
                ],
            ),
            (
                produce_history_vector(extract_numbers("0 3 6 9 12 15")),
                vec![
                    vec![-3, 0, 3, 6, 9, 12, 15],
                    vec![3, 3, 3, 3, 3, 3],
                    vec![0, 0, 0, 0, 0],
                ],
            ),
            (
                produce_history_vector(extract_numbers("1 3 6 10 15 21")),
                vec![
                    vec![0, 1, 3, 6, 10, 15, 21],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                ],
            ),
        ];

        for (mut input, expected) in tests {
            fill_history_vector(&mut input);
            assert_eq!(input, expected);
        }
    }
}
