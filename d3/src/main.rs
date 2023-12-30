use std::{fs, str::Chars};

fn main() {
    println!("Beep Boop - day three");

    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines: Vec<_> = content.lines().collect();
    let height = lines.len();
    let width = lines.get(0).unwrap().len();

    for row in 0..height {
        let line: Vec<_> = lines.get(row).unwrap().chars().collect();
        for col in 0..width {
            let c = line.get(col).unwrap();
            let is_symbol = c.is_ascii_alphanumeric() == false && c.clone() != '.';
        }
    }
}

fn find_adjacent(){}

fn parse_number(c: &Vec<&char>, init_index: usize) -> u32 {
    let mut start_index: usize = init_index;
    let end_index: usize;
    loop {
        start_index -= 1;
    }
}
