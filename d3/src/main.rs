use std::{fs, str::Chars};

fn main() {
    println!("Beep Boop - day three");

    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines: Vec<_> = content.lines().collect();
    let height = lines.len();
    let width = lines.get(0).unwrap().len();

    for line in lines {
        for c in line.chars() {
            let is_symbol = c.is_ascii_alphanumeric() == false && c != '.';
        }
    }
}

fn parse_number(c: &Vec<&char>, start_index: usize) -> u32 {

    loop {

    }
}
