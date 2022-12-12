use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// This is the main function
fn main() {
    // create elf counter
    let mut elf_counter = 1;
    let mut cal_counter: i32 = 0;
    let mut totals = Vec::new();
    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("./input.txt") else { return };
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(calories) = line {
            if calories.is_empty() {
                totals.push(cal_counter);
                elf_counter = elf_counter + 1;
                cal_counter = 0;
            } else {
                cal_counter = cal_counter + calories.parse::<i32>().unwrap();
                // println!("{}", cal_counter);
            }
        }
    } 
    totals.sort();
    totals.reverse();
    println!("Most Calories is {}", totals.get(0).unwrap());
    let top3 = totals.get(0).unwrap() + totals.get(1).unwrap() + totals.get(2).unwrap();
    println!("Top 3 Elfs {:?}", top3);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}