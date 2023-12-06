use std::{fs::File, io::Read};

pub fn get_data_chars(path: &str) -> Vec<Vec<char>> {
    let data = get_data(path);

    let mut data_chars: Vec<Vec<char>> = Vec::new();
    for line in data {
        if !line.is_empty() {
            data_chars.push(line.chars().collect());
        }
    }

    return data_chars;
}

pub fn get_data(path: &str) -> Vec<String> {
    let mut data = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("failed to read the data file: {:?}", err),
    };

    let mut content = String::new();
    data.read_to_string(&mut content)
        .ok()
        .expect("failed to parse file");

    let lines: Vec<String> = content
        .split("\n")
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();

    return lines;
}
