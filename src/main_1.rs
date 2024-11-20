use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_string_num(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse::<i32>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "src/numero.txt";
    let contents = read_file(filename)?;
    let number = parse_string_num(&contents)?;

    println!("O número é: {}", number);

    Ok(())
}
