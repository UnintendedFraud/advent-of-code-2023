#[derive(Debug, Clone, Copy)]
struct Number {
    row_idx: usize,
    start: usize,
    end: usize,
    line_len: usize,
    value: i32,
}

fn main() {
    let data_chars = util::get_data_chars("data1.txt");
    let data_chars2 = util::get_data_chars("data2.txt");

    let sum: i32 = get_sum(&data_chars);
    let sum2: i32 = get_gear_sum(&data_chars2);

    println!("## Total sum: {} ##", sum);
    println!("## Total sum2: {} ##", sum2);
}

fn get_gear_sum(data: &Vec<Vec<char>>) -> i32 {
    let gear_positions = get_gear_positions(data);

    let mut rows: Vec<Vec<Number>> = Vec::new();
    for (idx, row) in data.iter().enumerate() {
        rows.push(get_row_numbers(idx, row, data));
    }

    let mut sum: i32 = 0;

    gear_positions
        .iter()
        .map(|pos| get_gear_numbers(pos, &rows))
        .for_each(|numbers| {
            if numbers.len() == 2 {
                sum += numbers[0].value * numbers[1].value;
            }
        });

    return sum;
}

fn get_gear_numbers(gear: &(usize, usize), rows: &Vec<Vec<Number>>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    for r_idx in 0..rows.len() {
        for n_idx in 0..rows[r_idx].len() {
            if is_next_to_gear(gear, &rows[r_idx][n_idx]) {
                numbers.push(rows[r_idx][n_idx]);
            }
        }
    }

    return numbers;
}

fn is_next_to_gear(gear: &(usize, usize), n: &Number) -> bool {
    let start = if n.start > 0 { n.start - 1 } else { 0 };
    return gear.0.abs_diff(n.row_idx) < 2 && gear.1 >= start && gear.1 <= n.end + 1;
}

fn get_gear_positions(data: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();

    for x in 0..data.len() {
        for y in 0..data[x].len() {
            if data[x][y] == '*' {
                positions.push((x, y));
            }
        }
    }

    return positions;
}

fn get_sum(data: &Vec<Vec<char>>) -> i32 {
    let mut sum: i32 = 0;

    for (idx, row) in data.iter().enumerate() {
        sum += get_row_sum(idx, row, &data);
    }

    return sum;
}

fn get_row_numbers(row_idx: usize, row: &Vec<char>, data_chars: &Vec<Vec<char>>) -> Vec<Number> {
    let mut n = get_default_number(row_idx, data_chars[row_idx].len());

    let mut v: Vec<Number> = Vec::new();
    let mut digits: Vec<char> = Vec::new();

    for (idx, c) in row.iter().enumerate() {
        if c.is_digit(10) {
            if digits.len() == 0 {
                n.start = idx;
            }

            digits.push(*c);

            if idx == row.len() - 1 {
                n.end = idx;
                n.value = maybe_get_number(&n, &digits, data_chars);
                v.push(n);
            }
        } else if digits.len() > 0 {
            n.end = idx - 1;
            n.value = maybe_get_number(&n, &digits, data_chars);
            v.push(n);

            n = get_default_number(row_idx, data_chars[row_idx].len());
            digits = Vec::new();
        }
    }

    return v;
}

fn get_row_sum(row_idx: usize, row: &Vec<char>, data_chars: &Vec<Vec<char>>) -> i32 {
    let mut n = get_default_number(row_idx, data_chars[row_idx].len());

    let mut digits: Vec<char> = Vec::new();

    let mut sum: i32 = 0;

    for (idx, c) in row.iter().enumerate() {
        if c.is_digit(10) {
            if digits.len() == 0 {
                n.start = idx;
            }

            digits.push(*c);

            if idx == row.len() - 1 {
                n.end = idx;
                sum += maybe_get_number(&n, &digits, data_chars);
            }
        } else if digits.len() > 0 {
            n.end = idx - 1;
            sum += maybe_get_number(&n, &digits, &data_chars);

            n = get_default_number(row_idx, data_chars[row_idx].len());
            digits = Vec::new();
        }
    }

    return sum;
}

fn maybe_get_number(n: &Number, digits: &Vec<char>, data_chars: &Vec<Vec<char>>) -> i32 {
    if should_add_number(n, data_chars) {
        match digits.iter().collect::<String>().parse() {
            Ok(n) => return n,
            Err(err) => panic!("failed to parse number {:?}: {:?}", digits, err),
        }
    }

    return 0;
}

fn should_add_number(n: &Number, data_chars: &Vec<Vec<char>>) -> bool {
    let start = if n.start > 0 { n.start - 1 } else { n.start };
    let end = if n.end < n.line_len - 1 {
        n.end + 1
    } else {
        n.end
    };

    if n.row_idx > 0 && have_symbols_between(&data_chars[n.row_idx - 1], start, end)
        || have_symbols_between(&data_chars[n.row_idx], start, end)
        || n.row_idx < n.line_len - 1
            && have_symbols_between(&data_chars[n.row_idx + 1], start, end)
    {
        return true;
    }

    return false;
}

fn have_symbols_between(row: &Vec<char>, start: usize, end: usize) -> bool {
    if row.len() == 0 {
        return false;
    }

    for idx in start..end + 1 {
        if is_symbol(row[idx]) {
            return true;
        }
    }

    return false;
}

// I'm not sure what they mean by symbols
fn is_symbol(c: char) -> bool {
    return c != '.' && !c.is_digit(10);
}

fn get_default_number(row_idx: usize, length: usize) -> Number {
    return Number {
        row_idx,
        start: 0,
        end: 0,
        line_len: length,
        value: 0,
    };
}
