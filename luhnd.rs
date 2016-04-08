use std::env;

fn get_odd_even(string: &String, odd: bool) -> String {
    let modulus_result: i32 = if odd == true {
        1
    } else {
        0
    };

    let mut odds: String = "".to_string();

    let mut i = 1;
    for c in string.chars() {
        if i % 2 == modulus_result {
            let s: String = c.to_string();
            odds = odds + &s;
        }
        i = i + 1;
    }

    return odds;
}

fn sum_vector(vector: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in vector {
        sum = sum + i;
    }

    return sum;
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        panic!("Need a credit card number as input");
    } else {
        println!("Checking validity of {}...", args[1]);
    }

    let reversed: String = args[1].chars().rev().collect();

    let odds: String = get_odd_even(&reversed, true);

    let evens: String = get_odd_even(&reversed, false);

    // Sum odd digits to get s1
    let mut s1: i32 = 0;
    for c in odds.chars() {
        let int_char: i32 = c.to_string().parse().unwrap();
        s1 = s1 + int_char;
    }

    // convert evens string to vector
    let mut evens_vec = Vec::new();
    for c in evens.chars() {
        let int_char: i32 = c.to_string().parse().unwrap();
        evens_vec.push(int_char);
    }

    // Double evens
    for i in 0..evens_vec.len() {
        let num: i32 = evens_vec[i];
        evens_vec[i] = num * 2;
    }

    // Sum digits of each
    for i in 0..evens_vec.len() {
        let el: String = evens_vec[i].to_string();

        if el.len() > 1 {
            let mut sum: i32 = 0;
            for c in el.chars() {
                let c_int: i32 = c.to_string().parse().unwrap();
                sum = sum + c_int;
            }
            evens_vec[i] = sum;
        }
    }

    // Sum all to get s2
    let s2: i32 = sum_vector(evens_vec);

    // If s1 + s2 ends in 0, then pass the test
    let answer: String = (s1 + s2).to_string();

    let mut valid: bool = false;
    let mut i: i32 = 0;
    for c in answer.chars() {
        let length: i32 = answer.len() as i32;
        if i >= length - 1 && c == '0' {
            valid = true;
        }

        i = i + 1;
    }

    println!("valid? {}", valid);
}
