fn main() {
    let data = util::get_data("data.txt");

    let mut results = Vec::new();
    let mut results2 = Vec::new();

    for i in 0..data.len() {
        let r = process_row(&data[i]);

        results.push(r.0);
        results2.push(r.1);
    }

    println!("## Results part 1: {}", results.iter().sum::<i32>());
    println!("## Results part 2: {}", results2.iter().sum::<i32>());
}

fn process_row(row: &String) -> (i32, i32) {
    let numbers = util::get_numbers_from_str::<i32>(&row, " ");

    let mut v: Vec<Vec<i32>> = Vec::new();
    v.push(numbers);

    let mut finished = false;
    let mut i = 0;

    while !finished {
        let r = get_next_row(&v[i]);
        v.push(r.0);

        if r.1 {
            finished = true;
        }

        i += 1;
    }

    return (value_from_processed(&v), value_from_processed_part2(&v));
}

fn value_from_processed_part2(v: &Vec<Vec<i32>>) -> i32 {
    let mut tmp = 0;
    for i in (1..v.len()).rev() {
        let v_in = &v[i - 1];
        tmp = v_in[0] - tmp;
    }

    return tmp;
}

fn value_from_processed(v: &Vec<Vec<i32>>) -> i32 {
    let mut tmp = 0;
    for i in (1..v.len()).rev() {
        let v_in = &v[i - 1];
        tmp += v_in[v_in.len() - 1];
    }

    return tmp;
}

fn get_next_row(row: &Vec<i32>) -> (Vec<i32>, bool) {
    let mut next_row = Vec::new();
    let mut only_zeroes = true;

    for i in 0..(row.len() - 1) {
        let v = row[i + 1] - row[i];

        next_row.push(v);

        if v != 0 {
            only_zeroes = false;
        }
    }

    return (next_row, only_zeroes);
}
