pub fn get_total_points() -> u32 {
    let data1 = util::get_data("data1.txt");

    let mut sum: u32 = 0;

    for i in 0..data1.len() {
        let split: Vec<&str> = data1[i].split(":").collect();
        let numbers: Vec<&str> = split[1].split("|").collect();

        let winning = get_numbers_from_str(numbers[0]);
        let card = get_numbers_from_str(numbers[1]);

        sum += get_game_points(&winning, &card);
    }

    return sum;
}

fn get_game_points(w: &Vec<usize>, c: &Vec<usize>) -> u32 {
    let shared_len = c
        .iter()
        .filter(|x| w.contains(x))
        .collect::<Vec<&usize>>()
        .len();

    if shared_len == 0 {
        return 0;
    }

    let base: u32 = 2;

    return base.pow((shared_len - 1) as u32);
}

fn get_numbers_from_str(str: &str) -> Vec<usize> {
    return str
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse().expect("failed to parse"))
        .collect();
}
