use std::fs::File;
use std::io::{ BufReader };
use std::io::prelude::*;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let (mut sum_part1, mut sum_part2) = (0, 0);

    for line_ in reader.lines() {
        let line = line_.unwrap();

        let parts:Vec<&str> = line.split_whitespace().collect();

        let part1_scores = [
            [4, 8, 3],
            [1, 5, 9],
            [7, 2, 6],
        ];

        let part2_scores = [
            [3, 4, 8],
            [1, 5, 9],
            [2, 6, 7],
        ];

        sum_part1 += part1_scores[parts[0].chars().next().unwrap() as usize - 'A' as usize]
                     [parts[1].chars().next().unwrap() as usize - 'X' as usize];

        sum_part2 += part2_scores[parts[0].chars().next().unwrap() as usize - 'A' as usize]
            [parts[1].chars().next().unwrap() as usize - 'X' as usize];                     
    }

    println!("Sum part1: {}, sum part2: {}", sum_part1, sum_part2)
}
