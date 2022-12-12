use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const PRIORITY: [char; 52] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

fn main() {
    let mut line_count = 0;
    let mut total_score = 0;
    let mut group = Vec::new();
    let mut group_num = 1;
    let mut found = false;
    let Ok(lines) = read_lines("./input.txt") else { return };
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if line_count == 0 {
            println!("Group {}", group_num);
        }
        if let Ok(items) = line {
            println!("{}", items);
            group.push(items);
            line_count = line_count + 1;
        }
        if line_count == 3 {
            group_num = group_num + 1;
            for item in group[0].chars() {
                if group[1].contains(item) && group[2].contains(item) && found == false {
                    found = true;
                    let priority_value = PRIORITY.iter().position(|&x| x == item).unwrap() + 1;
                    total_score = total_score + priority_value;
                    println!("{} - {}", item, priority_value);
                }         
            }
            line_count = 0;
            found = false;
            group = Vec::new();
        }
    }
    println!("Total Score {}", total_score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}