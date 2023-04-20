use std::fs::File;
use std::io::{ BufReader };
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

fn item_value(item: char) -> i32 {
    if item >= 'a' && item <= 'z' {
        item as i32 - 'a' as i32 + 1
    }
    else {
        item as i32 - 'A' as i32 + 27
    }
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;
    for line_ in reader.lines() {
        let mut items: HashSet<char> = HashSet::new();
        let line = line_.unwrap();

        for (i,c) in line.chars().enumerate() {
            if i < line.len() / 2 {
                items.insert(c);
            } else {
                if items.contains(&c) {
                    score += item_value(c);
                    break;
                }
            }
        }       
    }    
    println!("Score: {}", score)
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;
    let mut line_num = 0;
    let mut items: HashMap<char, i32> = HashMap::new();
    for line_ in reader.lines() {
        let line = line_.unwrap();

        for c in line.chars() {
            let val = items.entry(c).or_insert(0);
            *val = *val | 1 << line_num;
        }

        line_num += 1;
        if line_num == 3 {
            line_num = 0;
            for (c,v) in &items {
                if *v == 7 {
                    score = score + item_value(*c);
                    break;
                }
            }

            items = HashMap::new();
        }
    }
    println!("Score: {}", score)
}

fn main() {
    part1();
    part2();
}
