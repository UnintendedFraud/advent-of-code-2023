struct Almanac {
    og_seeds: Vec<usize>,
    maps: Vec<Vec<Vec<usize>>>,
}

pub fn get_lowest_location_number() -> usize {
    let data = util::get_data("data.txt");

    let almanac = format_data(data);

    let mut locations = Vec::new();

    for i in 0..almanac.og_seeds.len() {
        locations.push(get_seed_location_number(
            almanac.og_seeds[i],
            &almanac.maps,
            0,
        ));
    }

    return *locations.iter().min().unwrap();
}

fn get_seed_location_number(seed: usize, maps: &Vec<Vec<Vec<usize>>>, idx: usize) -> usize {
    let v = get_group_value(seed, &maps[idx]);

    if idx < maps.len() - 1 {
        return get_seed_location_number(v, maps, idx + 1);
    }

    return v;
}

fn get_group_value(seed: usize, group: &Vec<Vec<usize>>) -> usize {
    // just in case stuff override other in the data
    for i in (0..group.len()).rev() {
        let g = &group[i];
        if seed >= g[1] && seed <= (g[1] + g[2]) {
            return g[0] + seed - g[1];
        }
    }

    return seed;
}

fn format_data(data: Vec<String>) -> Almanac {
    let mut og_seeds = Vec::new();
    let mut maps = Vec::new();

    for i in 0..data.len() {
        if i == 0 {
            og_seeds =
                util::get_numbers_from_str(data[i].split(":").collect::<Vec<&str>>()[1], " ");
        } else if data[i].contains("map:") {
            maps.push(Vec::new());
        } else if !data[i].contains("map:") {
            let row = util::get_numbers_from_str(data[i].as_str(), " ");
            let len = if maps.len() > 0 { maps.len() - 1 } else { 0 };
            maps[len].push(row);
        }
    }

    return Almanac { og_seeds, maps };
}
