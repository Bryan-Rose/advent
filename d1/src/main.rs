use regex::Regex;
use std::env;
use std::fs;

fn main() {
    println!("Beep Boop - day one");
    let dir = std::env::current_dir().expect("OS fucked up");
    let dirStr = dir.display();
    println!("{dirStr}");

    let contents: String =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();
    let re_num = Regex::new(r"[0-9]").unwrap();
    let mut running_total = 0;
    for line in lines {
        //println!("{line}");
        let caps = re_num.captures(line).unwrap();
        let num_caps = caps.len();
        let first: &str = caps.get(0).unwrap().as_str();
        let last: &str = caps.get(num_caps - 1).unwrap().as_str();
        let line_val: i32 = "{first}{last}".parse().unwrap();

        running_total += line_val;
    }

    println!("{running_total}");
}
