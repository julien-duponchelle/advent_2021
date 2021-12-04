use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut last = -1;
    let mut res = 0;

    // Unwrap is use to ignore the error 
    let file = File::open("input").unwrap();
    for line in io::BufReader::new(file).lines() {
        let nb = line.unwrap();
        let my_int: i32 = nb.parse().unwrap();
        if last != -1 && my_int > last {
            res += 1;
        }
        last = my_int;
    }
    println!("{}", res);
}

