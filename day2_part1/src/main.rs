use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut position = 0;
    let mut depth = 0;

    // Unwrap is use to ignore the error 
    let file = File::open("input").unwrap();
    for line in io::BufReader::new(file).lines() {
        let input = line.unwrap();
        let mut s = input.split_whitespace();
        let command = s.next().unwrap();
        let value: i32 = s.next().unwrap().parse().unwrap();
        println!("{} {}", command, value);

        if command == "forward" {
            position += value;
        }
        else if command == "down" {
            depth += value;
        }
        else if command == "up" {
            depth -= value;
        }
    }
    println!("{}", position * depth);
}

