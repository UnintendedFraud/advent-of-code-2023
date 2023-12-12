use std::collections::HashMap;

fn main() {
    let pipes = HashMap::from([
        ('|', (0, 1)),
        ('-', (1, 0)),
        ('L', (1, 1)),
        ('J', (1, -1)),
        ('7', (1, -1)),
        ('F', (1, 1)),
    ]);

    let data = util::get_data_chars("data.txt");

    let (start, starting_directions) = find_starting_directions(&data, &pipes);

    let max = go_through_pipe(&data, start, &starting_directions);

    println!("## part 1 max: {}", max);
}

fn go_through_pipe(
    d: &Vec<Vec<char>>,
    start: (i32, i32),
    starting_directions: &Vec<(i32, i32)>,
) -> i32 {
    let results = starting_directions
        .iter()
        .map(|direction| get_max_result(&d, start, direction))
        .collect::<Vec<i32>>();

    return *results.iter().max().unwrap();
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

fn find_starting_directions(
    d: &Vec<Vec<char>>,
    pipes: &HashMap<char, (i32, i32)>,
) -> ((i32, i32), Vec<(i32, i32)>) {
    let mut v = Vec::new();
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

    let possible_starts = vec![
        (start.0 - 1, start.1),
        (start.0, start.1 - 1),
        (start.0 + 1, start.1),
        (start.0, start.1 + 1),
    ];

    for i in 0..possible_starts.len() {
        let el = possible_starts[i];

        if el.0 >= 0 && el.1 >= 0 && pipes.contains_key(&d[el.0 as usize][el.1 as usize]) {
            v.push(el);
        }
    }

    return (start, v);
}
