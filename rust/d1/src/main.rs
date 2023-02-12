use std::fs::File;
use std::io::{ BufReader };
use std::io::prelude::*;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut max = 0;
    let mut sum = 0;
    
    for line_ in reader.lines() {
        let line = line_.unwrap();
        
        if line.trim().is_empty() == true {
            max = max.max(sum);
            sum = 0;
            continue;
        }
        let line_int: i32 = line.trim().parse().unwrap(); 
        sum += line_int; 
    }

    println!("Max: {}", max)
}