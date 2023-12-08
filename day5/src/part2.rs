// TODO: I believe there is a way to reduce the amount of seed values to test
// and therefore be fast, but couldn't figure it out

struct Almanac {
    seeds: Vec<usize>,
    seeds_ranges: Vec<[usize; 2]>,
    maps: Vec<Vec<Vec<usize>>>,
}

impl Almanac {
    fn seed_belongs(&self, seed: usize) -> bool {
        for i in 0..self.seeds_ranges.len() {
            let range = self.seeds_ranges[i];

            if seed >= range[0] && seed <= range[1] {
                return true;
            }
        }

        return false;
    }
}

pub fn get_lowest_location_number() -> usize {
    let data = util::get_data("data.txt");

    let almanac = format_data(data);

    let mut loc = 1000000;
    let mut loc_found_seed = false;

    while !loc_found_seed {
        let seed = find_seed_from_loc(loc, &almanac.maps, almanac.maps.len() - 1);

        if almanac.seed_belongs(seed) {
            loc_found_seed = true;
        } else {
            loc += 1;
        }
    }

    return loc;
}

fn find_seed_from_loc(value: usize, maps: &Vec<Vec<Vec<usize>>>, idx: usize) -> usize {
    let v = get_group_seed_from_value(value, &maps[idx]);

    if idx > 0 {
        return find_seed_from_loc(v, maps, idx - 1);
    }

    return v;
}

fn get_group_seed_from_value(value: usize, group: &Vec<Vec<usize>>) -> usize {
    for i in (0..group.len()).rev() {
        let g = &group[i];

        if value >= g[0] && value <= g[0] + g[2] - 1 {
            return g[1] + value - g[0];
        }
    }

    return value;
}

fn format_data(data: Vec<String>) -> Almanac {
    let mut seeds_ranges = Vec::new();
    let mut maps = Vec::new();

    for i in 0..data.len() {
        if i == 0 {
            seeds_ranges = get_seeds_ranges(&data[i]);
        } else if data[i].contains("map:") {
            maps.push(Vec::new());
        } else {
            let row = util::get_numbers_from_str(data[i].as_str(), " ");
            let len = if maps.len() > 0 { maps.len() - 1 } else { 0 };
            maps[len].push(row);
        }
    }

    let seeds = get_seeds_to_check(&seeds_ranges, &maps[0]);

    return Almanac {
        seeds,
        seeds_ranges,
        maps,
    };
}

fn get_seeds_to_check(ranges: &Vec<[usize; 2]>, group: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut seeds = Vec::new();

    for i in 0..group.len() {
        let g = &group[i];
        let soil_low = g[0];
        let soil_up = g[0] + g[2] - 1;

        for j in 0..ranges.len() {
            let r = ranges[j];
            let seed_low = r[0];
            let seed_up = r[1];

            if seed_low < soil_low && seed_up > soil_up {
                seeds.push(seed_low);
                seeds.push(soil_low);
                seeds.push(soil_up + 1);
            } else if seed_up < soil_low || seed_low > soil_up {
                seeds.push(seed_low);
            } else if seed_low < soil_low && seed_up > soil_low && seed_up < soil_up {
                seeds.push(seed_low);
                seeds.push(soil_low);
            } else if seed_low < soil_up && seed_up > soil_up {
                seeds.push(seed_low);
                seeds.push(soil_up + 1);
            } else if seed_low > soil_low && seed_up < soil_up {
                seeds.push(seed_low);
            }
        }
    }

    seeds.sort();
    seeds.dedup();

    println!("seeds: {:?}", seeds);
    return seeds;
}

fn get_seeds_ranges(s: &String) -> Vec<[usize; 2]> {
    let ranges = util::get_numbers_from_str(s.split(":").collect::<Vec<&str>>()[1], " ");

    let mut numbers = Vec::new();

    let mut i = 0;

    while i < ranges.len() - 1 {
        let start = ranges[i];
        let end = ranges[i] + ranges[i + 1] - 1;
        numbers.push([start, end]);

        i += 2;
    }

    return numbers;
}
