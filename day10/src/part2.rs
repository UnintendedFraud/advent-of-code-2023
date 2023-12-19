pub fn get_insides(d: &Vec<Vec<char>>, start: &(i32, i32), direction: &(i32, i32)) -> i32 {
    let path = get_path(d, start, direction);

    // the other sign combined with those counts as 1
    let turns_chars = Vec::from(['J', 'L', '|']);

    let mut count = 0;

    for i in 0..d.len() {
        let row = &d[i];

        let mut pipes_count = 0;

        for j in 0..row.len() {
            let c = if row[j] == 'S' { 'J' } else { row[j] };

            let is_char_path = is_path((i, j), &path);

            if is_char_path && turns_chars.contains(&c) {
                pipes_count += 1;
            } else if !is_char_path && pipes_count % 2 != 0 {
                count += 1;
            }
        }
    }

    return count;
}

fn get_path(d: &Vec<Vec<char>>, start: &(i32, i32), direction: &(i32, i32)) -> Vec<(usize, usize)> {
    let mut finished = false;

    let mut row = direction.0 as usize;
    let mut col = direction.1 as usize;

    let mut prev_row = start.0 as usize;
    let mut prev_col = start.1 as usize;

    let mut path = Vec::from([(prev_row, prev_col)]);

    while !finished {
        let next_char = &d[row][col];

        path.push((row, col));

        if *next_char == 'S' {
            finished = true;
        } else {
            let tmp_row = row;
            let tmp_col = col;

            match *next_char {
                '|' => {
                    if prev_row > row {
                        row -= 1;
                    } else {
                        row += 1;
                    }
                }
                '-' => {
                    if prev_col > col {
                        col -= 1;
                    } else {
                        col += 1;
                    }
                }
                'L' => {
                    if prev_col > col {
                        row -= 1;
                    } else {
                        col += 1;
                    }
                }
                'J' => {
                    if prev_col < col {
                        row -= 1;
                    } else {
                        col -= 1;
                    }
                }
                '7' => {
                    if prev_col < col {
                        row += 1;
                    } else {
                        col -= 1;
                    }
                }
                'F' => {
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

    return path;
}

fn is_path(pos: (usize, usize), path: &Vec<(usize, usize)>) -> bool {
    for i in 0..path.len() {
        let p = path[i];

        if p.0 == pos.0 && p.1 == pos.1 {
            return true;
        }
    }

    return false;
}

fn print_path_row(path: &Vec<(usize, usize)>, row: usize) {
    let r: Vec<&(usize, usize)> = path.iter().filter(|x| x.0 == row).collect();

    println!("row path: {:?}", r);
}
