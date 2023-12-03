use std::{fs::File, io::Read};

pub fn get_data(path: &str) -> Vec<String> {
    let mut data = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("failed to read the data file: {:?}", err),
    };

    let mut content = String::new();
    data.read_to_string(&mut content)
        .ok()
        .expect("failed to parse file");

    let lines: Vec<String> = content.split("\n").map(|l| l.to_string()).collect();

    return lines;
}
