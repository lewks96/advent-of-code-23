
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn day_one(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().map(|l| l.unwrap()).collect();

    let mut acc = 0;

    for line in lines {

        let mut digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        println!("Digits: {:?}", digits);
        if digits.len() == 1 {
            digits.push(*digits.first().unwrap());
        }

        let str_val= format!("{}{}", digits[0], digits[digits.len()-1]);
        //let t: String = line.chars().filter(|c| c.is_digit(10)).collect();
        let number: i32= str_val.parse().unwrap();

        println!("{:?}", number);
        acc += number
    }
        println!("Acc: {:?}", acc);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <day> <filename>", args[0]);
        return;
    }
    let day = &args[1];
    let filename = &args[2];

    println!("Hello, world!");
    if day == "1" {
        day_one(filename);
    }
}
