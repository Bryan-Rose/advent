use regex::Regex;
use std::fs;

fn main() {
    println!("Beep Boop - day four");
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines = content.lines();
    let mut games: Vec<Game> = Vec::new();
    let mut total_score_part_one: i32 = 0;
    for line in lines {
        let game_opt = parse_line(line);
        if game_opt.is_some() {
            let mut game = game_opt.unwrap();
            let game_score = game.calc_score_part_one();
            total_score_part_one += game_score;

            games.push(game);
        } else {
            println!("{line}");
        }
    }

    calc_multiples_part_two(&mut games);

    println!("{total_score_part_one}")
}

fn calc_multiples_part_two(games: &Vec<Game>) -> Vec<Game> {
    let mut r: Vec<Game> = Vec::new();
    r.copy_from_slice(games);
    for i in 0..games.len() {
        let mut g = games.get(i).unwrap();
        let winners = g.calc_num_winners_part_two();
        let end_index = (i + winners as usize).min(games.len());
        for j in i..end_index {
            let mut gc = games.get(j).unwrap();
            gc.multiple = 1;
        }
    }

    return r;
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
        multiple: 1,
        played_numbers: plays,
        winning_numbers: wins,
    });
}

struct Game {
    id: i32,
    multiple: i32,
    winning_numbers: Vec<i32>,
    played_numbers: Vec<i32>,
}

impl Game {
    fn calc_num_winners_part_two(&self) -> i32 {
        let mut count = 0;
        for w in &self.winning_numbers {
            if self.played_numbers.contains(&w) {
                count += 1;
            }
        }
        return count;
    }

    fn calc_score_part_one(&self) -> i32 {
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
