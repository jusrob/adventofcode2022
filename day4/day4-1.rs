use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let Ok(lines) = read_lines("./input.txt") else { return };
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(pairs) = line {
            println!("{}", pairs);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
