use regex::Regex;
use std::fs;

struct GameCubes {
    red: u32,
    green: u32,
    blue: u32,
}

struct GameDraws {
    id: u32,
    draws: Vec<GameCubes>,
}

fn main() {
    println!("Beep Boop - day two");

    let allowed = GameCubes {
        red: 12,
        blue: 14,
        green: 13,
    };

    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines = content.lines();

    let mut ids_total_part_one: u32 = 0;
    let mut power_total_part_two: u32 = 0;

    for line in lines {
        let opt_line = parse_line(line);
        if opt_line.is_none() {
            continue;
        }
        let game_line = opt_line.unwrap();
        if valid_line_part_one(&game_line, &allowed) {
            ids_total_part_one += game_line.id;
        }

        let min_set = min_cubes(&game_line);
        println!(
            "{} - {} red {} blue {} green",
            game_line.id, min_set.red, min_set.blue, min_set.green
        );
        let power = min_set.red * min_set.blue * min_set.green;
        power_total_part_two += power;
    }

    println!("Part 1 - {}", ids_total_part_one);
    println!("Part 2 - {}", power_total_part_two);
}

fn min_cubes(game: &GameDraws) -> GameCubes {
    let red = game
        .draws
        .iter()
        .map(|x| x.red)
        .filter(|x| x > &0)
        .max()
        .unwrap_or(1);
    let blue = game
        .draws
        .iter()
        .map(|x| x.blue)
        .filter(|x| x > &0)
        .max()
        .unwrap_or(1);
    let green = game
        .draws
        .iter()
        .map(|x| x.green)
        .filter(|x| x > &0)
        .max()
        .unwrap_or(1);

    return GameCubes {
        red: red,
        blue: blue,
        green: green,
    };
}

fn valid_line_part_one(game: &GameDraws, allowed: &GameCubes) -> bool {
    return game
        .draws
        .iter()
        .all(|g| g.red <= allowed.red && g.blue <= allowed.blue && g.green <= allowed.green);
}

fn parse_line(line: &str) -> Option<GameDraws> {
    //Game 8: 19 red, 1 green, 4 blue; 14 blue; 2 red, 3 blue
    if line.starts_with("Game") == false {
        return None;
    }

    let sections: Vec<_> = line.split(':').collect();
    let game_section = sections.get(0).unwrap();
    let results_section = sections.get(1).unwrap();

    let game_id = game_section
        .trim_start_matches(|c: char| c.is_alphabetic() || c.is_whitespace())
        .parse::<u32>()
        .unwrap();

    let mut draws: Vec<GameCubes> = Vec::new();
    for d in results_section.split(';') {
        draws.push(parse_draw(d));
    }

    return Some(GameDraws {
        id: game_id,
        draws: draws,
    });
}

fn parse_draw(d: &str) -> GameCubes {
    //19 red, 1 green, 4 blue

    let re_red = Regex::new("(?<num>[0-9]+) red").unwrap();
    let re_blue = Regex::new("(?<num>[0-9]+) blue").unwrap();
    let re_green = Regex::new("(?<num>[0-9]+) green").unwrap();

    let red = re_red
        .captures(d)
        .and_then(|c| {
            return c
                .name("num")
                .map_or(Some(0), |m| m.as_str().parse::<u32>().ok());
        })
        .unwrap_or(0);

    let green = re_green
        .captures(d)
        .and_then(|c| {
            return c
                .name("num")
                .map_or(Some(0), |m| m.as_str().parse::<u32>().ok());
        })
        .unwrap_or(0);

    let blue = re_blue
        .captures(d)
        .and_then(|c| {
            return c
                .name("num")
                .map_or(Some(0), |m| m.as_str().parse::<u32>().ok());
        })
        .unwrap_or(0);

    return GameCubes {
        red: red,
        blue: blue,
        green: green,
    };
}
