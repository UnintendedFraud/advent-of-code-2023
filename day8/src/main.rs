use lcmx::lcmx;
use std::collections::HashMap;

#[derive(Debug)]
struct Instructions {
    directions: Vec<usize>,
    map: HashMap<String, [String; 2]>,
    starting_positions: Vec<String>,
}

fn main() {
    let instructions = format_data(&util::get_data("data.txt"));

    println!("starting: {:?}", instructions.starting_positions);

    let steps_count = get_to_zzz(&instructions, String::from("AAA"), 0, 0);
    let steps_count2 = get_to_xxz(&instructions);

    println!("## Total steps: {} ##", steps_count);
    println!("## Total steps 2: {} ##", steps_count2);
}

fn get_to_xxz(instructions: &Instructions) -> i128 {
    let mut scores: Vec<i128> = Vec::new();

    for i in 0..instructions.starting_positions.len() {
        let mut count = 0;
        let mut idx = 0;
        let mut finished = false;

        let mut key = String::from(&instructions.starting_positions[i]);

        while !finished {
            count += 1;

            let v = instructions.map.get(&key).unwrap();
            let d = instructions.directions[idx];

            key = String::from(&v[d]);

            if key.chars().last().unwrap() == 'Z' {
                finished = true;
            } else {
                idx = if idx + 1 == instructions.directions.len() {
                    0
                } else {
                    idx + 1
                };
            }
        }

        scores.push(count);
    }

    return lcmx(&scores).unwrap();
}

fn get_to_zzz(
    instructions: &Instructions,
    key: String,
    direction_idx: usize,
    count: usize,
) -> usize {
    let v = instructions.map.get(&key).unwrap();

    let idx = if direction_idx == instructions.directions.len() {
        0
    } else {
        direction_idx
    };
    let d = instructions.directions[idx];

    let new_key = &v[d];
    let new_count = count + 1;

    if new_key == "ZZZ" {
        return new_count;
    }

    return get_to_zzz(instructions, String::from(new_key), idx + 1, new_count);
}

fn format_data(data: &Vec<String>) -> Instructions {
    let mut directions = Vec::new();
    let mut map = HashMap::new();
    let mut starting_positions = Vec::new();

    for i in 0..data.len() {
        let line = &data[i];

        if i == 0 {
            directions = line
                .chars()
                .map(|c| {
                    if c == 'L' {
                        return 0;
                    } else {
                        return 1;
                    }
                })
                .collect();
        } else {
            let split: Vec<&str> = line.split("=").map(|x| x.trim()).collect();
            let k = split[0].to_string();
            let v = get_row_value(split[1].to_string());

            let kk = k.clone();
            if kk.chars().last().unwrap() == 'A' {
                starting_positions.push(kk);
            }

            map.insert(k, v);
        }
    }

    return Instructions {
        directions,
        map,
        starting_positions,
    };
}

fn get_row_value(s: String) -> [String; 2] {
    let mut tmp = s.replace("(", "");
    tmp = tmp.replace(")", "");
    tmp = tmp.replace(",", "");

    let split: Vec<&str> = tmp.split(" ").collect();

    return [split[0].to_string(), split[1].to_string()];
}
