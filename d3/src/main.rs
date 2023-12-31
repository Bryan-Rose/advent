use std::{fs};

struct Point {
    row: usize,
    col: usize,
}

fn main() {
    println!("Beep Boop - day three");

    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines: Vec<Vec<char>> = content
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let height = lines.len();
    let width = lines.get(0).unwrap().len();
    let max = Point {
        col: width,
        row: height,
    };

    let mut sum: u32 = 0;
    for row in 0..height {
        let line = lines.get(row).unwrap();
        for col in 0..width {
            let c = line.get(col).unwrap();
            let is_symbol = c.is_ascii_alphanumeric() == false && c.clone() != '.';
            if is_symbol {
                sum += find_numbers(&lines, &Point { row: row, col: col }, &max)
                    .iter()
                    .fold(0, |a, b| a + b);

                for n in find_numbers(&lines, &Point { row: row, col: col }, &max) {
                    println!("{n}");
                }
            }
        }
    }

    println!("{sum}");
}

fn find_numbers(arr: &Vec<Vec<char>>, loc: &Point, max: &Point) -> Vec<u32> {
    let mut r: Vec<u32> = Vec::new();
    let adj = find_adjacent(&arr, loc, &max);

    for a in adj {
        r.push(parse_number(arr.get(a.row).unwrap(), a.col));
    }

    return r;
}

fn valid_point(row: i32, col: i32, max: &Point) -> Option<Point> {
    if col >= 0 && (col as usize) < max.col && row >= 0 && (row as usize) < max.row {
        return Some(Point {
            row: row as usize,
            col: row as usize,
        });
    } else {
        return None;
    }
}

fn find_adjacent(arr: &Vec<Vec<char>>, loc: &Point, max: &Point) -> Vec<Point> {
    let mut r: Vec<Point> = Vec::new();

    let mut row: i32;
    let mut col: i32;

    // Up Left
    row = i32::try_from(loc.row).expect("loc.row larger than i32") - 1;
    col = i32::try_from(loc.col).expect("loc.col larger than i32") - 1;
    match valid_point(row, col, max) {
        Some(x) => {
            let check_char = arr.get(x.row).unwrap().get(x.col).unwrap();
            if check_char.is_numeric() {
                r.push(x);
            }
        }
        None => {}
    };

    // Left
    row = i32::try_from(loc.row).expect("loc.row larger than i32");
    col = i32::try_from(loc.col).expect("loc.col larger than i32") - 1;
    match valid_point(row, col, max) {
        Some(x) => {
            let check_char = arr.get(x.row).unwrap().get(x.col).unwrap();
            if check_char.is_numeric() {
                r.push(x);
            }
        }
        None => {}
    };

    // Down Left
    row = i32::try_from(loc.row).expect("loc.row larger than i32") + 1;
    col = i32::try_from(loc.col).expect("loc.col larger than i32") - 1;
    match valid_point(row, col, max) {
        Some(x) => {
            let check_char = arr.get(x.row).unwrap().get(x.col).unwrap();
            if check_char.is_numeric() {
                r.push(x);
            }
        }
        None => {}
    };

    // Down
    row = i32::try_from(loc.row).expect("loc.row larger than i32") + 1;
    col = i32::try_from(loc.col).expect("loc.col larger than i32");
    match valid_point(row, col, max) {
        Some(x) => {
            let check_char = arr.get(x.row).unwrap().get(x.col).unwrap();
            if check_char.is_numeric() {
                r.push(x);
            }
        }
        None => {}
    };

    // Down Right
    row = i32::try_from(loc.row).expect("loc.row larger than i32") + 1;
    col = i32::try_from(loc.col).expect("loc.col larger than i32") + 1;
    match valid_point(row, col, max) {
        Some(x) => {
            let check_char = arr.get(x.row).unwrap().get(x.col).unwrap();
            if check_char.is_numeric() {
                r.push(x);
            }
        }
        None => {}
    };

    // Right
    row = i32::try_from(loc.row).expect("loc.row larger than i32");
    col = i32::try_from(loc.col).expect("loc.col larger than i32") + 1;
    match valid_point(row, col, max) {
        Some(x) => {
            let check_char = arr.get(x.row).unwrap().get(x.col).unwrap();
            if check_char.is_numeric() {
                r.push(x);
            }
        }
        None => {}
    };

    // Up Right
    row = i32::try_from(loc.row).expect("loc.row larger than i32") - 1;
    col = i32::try_from(loc.col).expect("loc.col larger than i32") + 1;
    match valid_point(row, col, max) {
        Some(x) => {
            let check_char = arr.get(x.row).unwrap().get(x.col).unwrap();
            if check_char.is_numeric() {
                r.push(x);
            }
        }
        None => {}
    };

    // Up
    row = i32::try_from(loc.row).expect("loc.row larger than i32") - 1;
    col = i32::try_from(loc.col).expect("loc.col larger than i32");
    match valid_point(row, col, max) {
        Some(x) => {
            let check_char = arr.get(x.row).unwrap().get(x.col).unwrap();
            if check_char.is_numeric() {
                r.push(x);
            }
        }
        None => {}
    };

    r.sort_by_key(|x| x.row);
    r.dedup_by_key(|x| x).;

    return r;
}

fn parse_number(c: &Vec<char>, init_index: usize) -> u32 {
    let mut start_index: usize = init_index;
    let mut end_index: usize = init_index;

    loop {
        if c.get(start_index).unwrap().is_numeric() == false {
            start_index += 1;
            break;
        }

        if start_index == 0 {
            break;
        }

        start_index -= 1;
    }

    loop {
        end_index += 1;
        if end_index >= c.len() || c.get(end_index).unwrap().is_numeric() == false {
            end_index -= 1;
            break;
        }
    }

    let num_str: String = c[start_index..=end_index].iter().collect();
    let num: u32 = num_str.parse().unwrap();
    return num;
}
