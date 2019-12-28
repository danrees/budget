use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

fn main() {
    let path = Path::new("/home/dan/financial/simplii/SIMPLII.csv");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let f = BufReader::new(&file);

    for line in f.lines() {
        let l = line.unwrap();
        println!("{}",l)
    }
}
