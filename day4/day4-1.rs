use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut matches = 0;
    let Ok(lines) = read_lines("./input.txt") else { return };
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(pairs) = line {
            let (left, right) = pairs.split_once(",").unwrap();
            let (left_min, left_max) = left.split_once("-").unwrap();
            let (right_min, right_max) = right.split_once("-").unwrap();
            if int(left_min) <= int(right_min) && int(left_max) >= int(right_max){
                    matches = matches + 1;
            } else if int(right_min) <= int(left_min) && int(right_max) >= int(left_max) {
                matches = matches + 1;
            }
        }
    }
    println!("{} Matches", matches);
}
fn int(string: &str) -> i32{
    return string.parse::<i32>().unwrap()
}
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
