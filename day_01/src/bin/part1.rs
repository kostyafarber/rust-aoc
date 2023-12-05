use std::cmp::max;

fn main() {
    let input = include_str!("input.txt");

    let mut first_digit = 0;
    let mut second_digit = 0;

    let mut calibration_sum = 0;
    for c in input.chars() {
        if c.is_digit(10) {
            if first_digit == 0 {
                first_digit = c.to_digit(10).unwrap();
            } else {
                second_digit = c.to_digit(10).unwrap();
            }
        }

        if c == '\n' {
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
    }
    println!("The calibration sum for the document is: {calibration_sum}")
}
