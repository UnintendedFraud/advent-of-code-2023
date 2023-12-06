use std::collections::HashMap;

pub fn get_total_cards() -> usize {
    let data = util::get_data("data2.txt");

    let mut m: HashMap<usize, usize> = HashMap::new();

    for i in 0..data.len() {
        m.entry(i).and_modify(|v| *v += 1).or_insert(1);

        let count = get_count(&m, i);

        let split: Vec<&str> = data[i].split(":").collect();
        let numbers: Vec<&str> = split[1].split("|").collect();

        let winning = get_numbers_from_str(numbers[0]);
        let card = get_numbers_from_str(numbers[1]);

        let shared_len = card
            .iter()
            .filter(|x| winning.contains(x))
            .collect::<Vec<&usize>>()
            .len();

        for j in 0..shared_len {
            let k = i + j + 1;
            if k < data.len() {
                m.entry(k).and_modify(|v| *v += count).or_insert(count);
            }
        }
    }

    return m.iter().map(|(_, v)| v).sum();
}

fn get_count(m: &HashMap<usize, usize>, i: usize) -> usize {
    let count = match m.get(&i) {
        Some(n) => n,
        None => {
            println!("should always have a value, {:?}, {:?}", m, i);
            return 0;
        }
    };

    return *count;
}

fn get_numbers_from_str(str: &str) -> Vec<usize> {
    return str
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse().expect("failed to parse"))
        .collect();
}
