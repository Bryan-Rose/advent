use regex::Captures;
use regex::Regex;
use std::collections::HashMap;
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

        let both = first + last;
        println!("{both}");
        let line_val: u32 = both.parse().unwrap();

        running_total += line_val;
    }

    println!("{running_total}");
}

fn parse_line(line: &str) -> &u32 {
    let searches = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ];

    searches.map(|s| line.match_indices(s));

    for (s, i) in searches {
        //line.find(s)
    }
    //line.find(pat)
    return 1;

}

fn gather_line(line: &str) -> Iterator<u32, u32> {
    let searches = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "zero",
    ];
    for s in searches {
        line.find(s)
    }
}
