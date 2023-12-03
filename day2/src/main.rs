use std::{collections::HashMap, panic};

use util::get_data;

fn main() {
    let data = get_data("data2.txt");

    let mut games: HashMap<i32, HashMap<String, i32>> = HashMap::new();

    for line in data {
        if line.is_empty() {
            continue;
        }

        let game_split: Vec<&str> = line.split(":").collect();
        let game_n = get_game_number(game_split[0]);
        let colors = get_game_max_colors(game_split[1]);

        games.insert(game_n, colors);
    }

    let allowed_red = 12;
    let allowed_green = 13;
    let allowed_blue = 14;

    let mut count = 0;
    let mut count2 = 0;

    for (k, v) in games {
        let red = get_color_count(&v, "red");
        let green = get_color_count(&v, "green");
        let blue = get_color_count(&v, "blue");

        if red <= allowed_red && green <= allowed_green && blue <= allowed_blue {
            count += k;
        }

        count2 += red * green * blue;
    }

    println!("## Sum of valid games ID: {}", count);
    println!("## Part 2: {}", count2);
}

fn get_color_count(h: &HashMap<String, i32>, key: &str) -> i32 {
    let n: i32 = match h.get(key) {
        Some(g) => *g,
        None => 0,
    };

    return n;
}

fn get_game_max_colors(s: &str) -> HashMap<String, i32> {
    let mut h: HashMap<String, i32> = HashMap::from([
        (String::from("red"), 0),
        (String::from("green"), 0),
        (String::from("blue"), 0),
    ]);

    let group_colors: Vec<&str> = s.trim().split(";").collect();

    for group in group_colors {
        let colors: Vec<&str> = group.trim().split(",").collect();

        for color in colors {
            let c: Vec<&str> = color.trim().split(" ").collect();

            let n: i32 = match c[0].parse() {
                Ok(n) => n,
                Err(err) => panic!("failed to parse color count: {:?}", err),
            };

            let existing_count = get_color_count(&h, c[1]);

            if existing_count < n {
                h.insert(String::from(c[1]), n);
            }
        }
    }

    return h;
}

// fn get_game_sum_colors(s: &str) -> HashMap<String, i32> {
//     let mut h: HashMap<String, i32> = HashMap::from([
//         (String::from("red"), 0),
//         (String::from("green"), 0),
//         (String::from("blue"), 0),
//     ]);
//
//     let group_colors: Vec<&str> = s.trim().split(";").collect();
//
//     for group in group_colors {
//         let colors: Vec<&str> = group.trim().split(",").collect();
//
//         for color in colors {
//             let c: Vec<&str> = color.trim().split(" ").collect();
//
//             let n: i32 = match c[0].parse() {
//                 Ok(n) => n,
//                 Err(err) => panic!("failed to parse color count: {:?}", err),
//             };
//
//             h.entry(String::from(c[1])).and_modify(|v| *v = *v + n);
//         }
//     }
//
//     return h;
// }

fn get_game_number(s: &str) -> i32 {
    let s_split: Vec<&str> = s.trim().split(" ").collect();

    let game_number = match s_split.last() {
        Some(c) => *c,
        None => panic!("failed to get game number: {}", s),
    };

    let number: i32 = match String::from(game_number).parse() {
        Ok(n) => n,
        Err(err) => panic!("failed to parse the game number: {:?}", err),
    };

    return number;
}
