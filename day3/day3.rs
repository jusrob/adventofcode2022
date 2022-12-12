use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const PRIORITY: [char; 52] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

fn main() {
    let mut total_score = 0;
    let Ok(lines) = read_lines("./input.txt") else { return };
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(items) = line {
            let mut found_item = false;
            let length = items.len();
            let half = length/2;
            let (comp1, comp2) = items.split_at(half);
            // println!("{}", items);
            for item in comp1.chars() {
                // print!("{}", item);
                if comp2.contains(item) && found_item == false {
                    found_item = true;
                    let priority_value = PRIORITY.iter().position(|&x| x == item).unwrap() + 1;
                    println!("{} - {} - {} - {}", comp1, comp2, item, priority_value);
                    total_score = total_score + priority_value;
                }
            }
            // println!(" ");
        }
    }
    println!("Total Score - {}", total_score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}