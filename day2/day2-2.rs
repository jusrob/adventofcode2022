use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total_score = 0;
    let Ok(lines) = read_lines("./input.txt") else { return };
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(game) = line {
            let (left, right) = game.split_once(" ").unwrap();
            // println!("left = {} right = {}", left, right);
            total_score = total_score + points_won(left, right);
        }
    }
    println!("Total Score {}", total_score);
}

fn points_won(left: &str, right: &str) -> i32{
    let mut score = 0;
    if left == "A" {
        score = match right {
            "X" => score + 3,
            "Y" => score + 4,
            "Z" => score + 8,
            &_ => todo!(),
        };
    } else if left == "B" {
        score = match right {
            "X" => score + 1,
            "Y" => score + 5,
            "Z" => score + 9,
            &_ => todo!(),
        };
    } else if left == "C" {
        score = match right {
            "X" => score + 2,
            "Y" => score + 6,
            "Z" => score + 7,
            &_ => todo!(),
        };
    }
    // if right == "X" {
    //     score = score + 1;
    // } else if right == "Y" {
    //     score = score + 2;
    // } else if right == "Z" {
    //     score = score + 3;
    // }
    // println!("{} vs {}", left, right);
    println!("Score: {}", score);
    return score;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}