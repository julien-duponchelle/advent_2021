use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut fishes: Vec<i32> = Vec::new();

    let file = File::open("input").unwrap();
    for line in io::BufReader::new(file).lines() {
        for fish in line.unwrap().split(',') {
            fishes.push(fish.parse().unwrap());
        }
    }

    let mut day = 0;
    while day < 80 {
        let mut fishes_updated: Vec<i32> = Vec::new();
        while fishes.len() > 0 {
            let fish = fishes.pop().unwrap();
            if fish == 0 {
                fishes_updated.push(6);
                fishes_updated.insert(0, 8);
            }
            else {
                fishes_updated.push(fish - 1);
            }
        }
        fishes_updated.reverse();
        fishes = fishes_updated;
        day += 1
    }

    println!("{}", fishes.len());
}
