use std::io::{self};

fn main() {
    println!("Enter a positive input btwn 1-999,999,999", );
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input = input[0..input.len()-1].to_string(); //by default read_line adds \n, this cuts out

    let mut input_int: u32;
    let mut output = String::new();

    match input.parse::<u32>() {
        Ok(n) => input_int = n,
        Err(e) => return println!("Input has to be a positive integer."),
    }

    // first / middle / last like 999 / 888 / 777 in 999,888,777
    // one / two like 1 / 23 in 123

    let first_one = first_digit(String::new(), round_down(input_int/100000000));
    let first_two = second_two_digits(String::new(), (input_int % 100000000)/1000000);

    let middle_one = first_digit(String::new(), round_down((input_int%1000000)/100000));
    let middle_two = second_two_digits(String::new(), (input_int % 100000)/1000);

    let last_one = first_digit(String::new(), round_down((input_int % 1000)/100));
    let last_two = second_two_digits(String::new(), input_int % 100);

    if input.len() <= 3 {
        output = [last_one, last_two].concat();
    } else if input.len() <= 6 {
        output = [
            middle_one, middle_two,
            " thousand, ".to_string(),
            last_one, last_two].concat();
    } else if input.len() <= 9 {
        output = [
            first_one, first_two,
            " million, ".to_string(),
            middle_one, middle_two,
            " thousand, ".to_string(),
            last_one, last_two].concat();
    } else {
        output = String::from("too large!")
    }

    println!("input: {:?}", input);
    println!("output: {:?}", output.trim());
}

/*** UTILITY FUNCTIONS ***/


fn round_down(number: u32) -> u32 {
    ((number as f64).floor() as u32)
}

/*** FIRST DIGIT GENERATOR ***/

fn first_digit(mut first_storage: String, number: u32) -> String {
    if number == 0 {
        return first_storage;
    }
    first_storage = [ones(number).trim(), " hundred "].concat();
    first_storage
}


/*** SECOND TWO DIGIT GENERATION ***/

fn second_two_digits(mut second_two_storage: String, number: u32) -> String {
    if number < 20 {
        second_two_storage = ones(number).to_string();
    } else {
        let first = tens(round_down(number / 10));
        let second = ones(number % 10);
        second_two_storage = [first, second].concat().trim().to_string();
    }
    second_two_storage
}

fn ones(number: u32) -> &'static str {
    if number == 0 {
        return "";
    }
    return match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    };
}

fn tens(number: u32) -> &'static str {
    return match number {
        2 => "twenty ",
        3 => "thirty ",
        4 => "fourty ",
        5 => "fifty ",
        6 => "sixty ",
        7 => "seventy ",
        8 => "eighty ",
        9 => "ninety ",
        _ => "",
    }
}
