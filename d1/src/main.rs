use std::fs;

fn main() {
    println!("Beep Boop - day one");
    let dir = std::env::current_dir().expect("OS fucked up");
    let dir_str = dir.display();
    println!("{dir_str}");

    let contents: String =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let mut running_total = 0;
    for line in lines {
        let num = parse_line(line);
        if num.is_none() {
            continue;
        }
        let nnum = num.unwrap();

        println!("{nnum}");

        running_total += nnum;
    }

    println!("{running_total}");
}

fn parse_line(line: &str) -> Option<u32> {
    let mut matches = get_matches(line);
    if matches.len() == 0 {
        return None;
    }
    matches.sort_by(|a, b| a.0.cmp(&b.0));
    let first = matches.get(0).unwrap().1;
    let last = matches.last().unwrap().1;
    let line_num = (first * 10) + last;
    return Some(line_num);
}

fn get_matches(line: &str) -> Vec<SearchMatch> {
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

    let mut matches: Vec<SearchMatch> = Vec::new();

    for (s, i) in searches {
        let line_matches: Vec<(usize, &str)> = line.match_indices(s).collect();
        for li in line_matches {
            matches.push((li.0, i));
        }
    }

    return matches;
}

// First is index, second is digit
type SearchMatch = (usize, u32);
