use regex::Captures;
use regex::Regex;
use std::env;
use std::fs;

fn main() {
    println!("Beep Boop - day one");
    let dir = std::env::current_dir().expect("OS fucked up");
    let dir_str = dir.display();
    println!("{dir_str}");

    let contents: String =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let re_num = Regex::new(r"[0-9]").unwrap();
    let mut running_total = 0;
    for line in lines {
        let caps: Vec<Captures<'_>> = re_num.captures_iter(line).collect();
        if caps.is_empty() {
            continue;
        }

        let first: String = caps.first().unwrap().get(0).unwrap().as_str().to_owned();
        let last: &str = caps.last().unwrap().get(0).unwrap().as_str();

        // let first: String = caps.get(0).unwrap().as_str().to_owned();
        // let last: &str = caps.get(caps.len() - 1).unwrap().as_str();
        let both = first + last;
        println!("{both}");
        let line_val: u32 = both.parse().unwrap();

        running_total += line_val;
    }

    println!("{running_total}");
}
