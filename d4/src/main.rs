use regex::Regex;
use std::fs;

fn main() {
    println!("Beep Boop - day four");
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines = content.lines();
    let mut total_score: i32 = 0;
    for line in lines {
        let game_opt = parse_line(line);
        if game_opt.is_some() {
            let game = game_opt.unwrap();
            let id = game.id;
            let game_score = game.calc_score();
            println!("{id} {game_score}");
            total_score += game_score;
        }else {
            println!("{line}");
        }
    }

    println!("{total_score}")
}

fn parse_line(line: &str) -> Option<Game> {
    //Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

    let re = Regex::new(r"Card +(?<id>\d+): +(?<wins>.+)\| +(?<plays>.+)").unwrap();
    let caps: Vec<_> = re.captures_iter(line).collect();

    if caps.len() != 1 {
        return None;
    }

    let cap = caps.get(0).unwrap();

    let id: i32 = cap.name("id").unwrap().as_str().parse().unwrap();

    let wins_str = cap.name("wins").unwrap().as_str();
    let wins: Vec<i32> = wins_str
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let plays_str = cap.name("plays").unwrap().as_str();
    let plays: Vec<i32> = plays_str
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    return Some(Game {
        id: id,
        played_numbers: plays,
        winning_numbers: wins,
    });
}

struct Game {
    id: i32,
    winning_numbers: Vec<i32>,
    played_numbers: Vec<i32>,
}

impl Game {
    fn calc_score(&self) -> i32 {
        let mut count = 0;
        let mut score = 0;
        for w in &self.winning_numbers {
            if self.played_numbers.contains(&w) {
                count += 1;
                if count == 1 {
                    score = 1;
                } else {
                    score = score * 2;
                }
            }
        }
        return score;
    }
}
