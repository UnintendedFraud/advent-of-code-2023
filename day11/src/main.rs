fn main() {
    let part1 = get_result(1);
    let part2 = get_result(1_000_000);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn get_result(n: i64) -> i64 {
    let data = util::get_data_chars("data.txt");
    let (empty_rows, empty_cols) = get_empty_data(&data);

    let galaxies_positions = get_galaxies(&data);

    let mut distances = Vec::new();

    for i in 0..galaxies_positions.len() - 1 {
        let (row, col) = galaxies_positions[i];

        for j in i + 1..galaxies_positions.len() {
            let (next_row, next_col) = galaxies_positions[j];

            let diff_col = get_diff(col, next_col, n, &empty_cols);
            let diff_row = get_diff(row, next_row, n, &empty_rows);

            distances.push(diff_col + diff_row);
        }
    }

    return distances.iter().sum();
}

fn get_diff(current: usize, next: usize, n: i64, empty: &Vec<usize>) -> i64 {
    let high;
    let low;

    if current > next {
        high = current;
        low = next;
    } else {
        high = next;
        low = current;
    }

    let mut empty_stuff = 0;

    for i in 0..empty.len() {
        if empty[i] > low && empty[i] < high {
            empty_stuff += 1;
        }
    }

    let coeff = if n > 1 { n - 1 } else { n };

    return (high - low) as i64 + coeff * empty_stuff;
}

fn get_galaxies(data: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == '#' {
                positions.push((i, j));
            }
        }
    }

    return positions;
}

fn get_empty_data(data: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = Vec::new();
    let mut filled_cols = Vec::new();

    for i in 0..data.len() {
        let r = &data[i];

        let mut is_empty_row = true;

        for j in 0..r.len() {
            let c = r[j];

            if c != '.' {
                if is_empty_row {
                    is_empty_row = false;
                }

                if !filled_cols.contains(&j) {
                    filled_cols.push(j);
                }
            }
        }

        if is_empty_row {
            empty_rows.push(i);
        }
    }

    let mut empty_cols = Vec::new();
    let row_size = data[0].len();

    for i in 0..row_size {
        if !filled_cols.contains(&i) {
            empty_cols.push(i);
        }
    }

    return (empty_rows, empty_cols);
}
