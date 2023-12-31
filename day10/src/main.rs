mod part2;

fn main() {
    let data = util::get_data_chars("data.txt");

    let (start, starting_direction) = find_starting_directions(&data);

    let max = get_max_result(&data, start, &starting_direction);
    let inside_count = part2::get_insides(&data, &start, &starting_direction);

    println!("## part 1 max: {}", max);
    println!("## part 2 insides: {}", inside_count);
}

fn get_max_result(d: &Vec<Vec<char>>, start: (i32, i32), direction: &(i32, i32)) -> i32 {
    let mut count = 1;
    let mut finished = false;

    let mut row = direction.0 as usize;
    let mut col = direction.1 as usize;

    let mut prev_row = start.0 as usize;
    let mut prev_col = start.1 as usize;

    while !finished {
        let next_char = &d[row][col];

        if *next_char == 'S' {
            finished = true;
        } else {
            let tmp_row = row;
            let tmp_col = col;

            match *next_char {
                '|' => {
                    count += 1;

                    if prev_row > row {
                        row -= 1;
                    } else {
                        row += 1;
                    }
                }
                '-' => {
                    count += 1;
                    if prev_col > col {
                        col -= 1;
                    } else {
                        col += 1;
                    }
                }
                'L' => {
                    count += 1;

                    if prev_col > col {
                        row -= 1;
                    } else {
                        col += 1;
                    }
                }
                'J' => {
                    count += 1;

                    if prev_col < col {
                        row -= 1;
                    } else {
                        col -= 1;
                    }
                }
                '7' => {
                    count += 1;

                    if prev_col < col {
                        row += 1;
                    } else {
                        col -= 1;
                    }
                }
                'F' => {
                    count += 1;

                    if prev_col > col {
                        row += 1;
                    } else {
                        col += 1;
                    }
                }
                _ => {
                    println!("unexpected char: {}", next_char);
                    break;
                }
            };

            prev_row = tmp_row;
            prev_col = tmp_col;
        }
    }

    return count / 2;
}

fn find_starting_directions(d: &Vec<Vec<char>>) -> ((i32, i32), (i32, i32)) {
    let mut start: (i32, i32) = (-1, -1);

    for i in 0..d.len() {
        let row = &d[i];

        for j in 0..row.len() {
            if row[j] == 'S' {
                start = (i as i32, j as i32);
            }
        }
    }

    if start.0 == -1 {
        panic!("failed to find the starting position");
    }

    // find a possible start

    let possible_starts = vec![
        ((start.0 - 1, start.1), vec!['|', '7', 'F']),
        ((start.0, start.1 - 1), vec!['-', 'F', 'L']),
        ((start.0, start.1 + 1), vec!['-', '7', 'J']),
        ((start.0 + 1, start.1), vec!['|', 'J', 'L']),
    ];

    for i in 0..possible_starts.len() {
        let pos = &possible_starts[i].0;
        let signs = &possible_starts[i].1;

        if pos.0 >= 0 && pos.1 >= 0 && signs.contains(&d[pos.0 as usize][pos.1 as usize]) {
            return (start, *pos);
        }
    }

    panic!("failed to find a starting position");
}
