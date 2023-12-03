use util::get_data;

fn main() {
    let word_numbers: Vec<String> = vec![
        String::from("zero"),
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
    ];
    let data = get_data("data.txt");

    let mut values: Vec<u32> = Vec::new();

    for n in 0..data.len() {
        let line = &data[n];

        let digits = format!(
            "{}{}",
            get_first_digit(&word_numbers, line),
            get_last_digit(&word_numbers, line)
        );

        let combined: u32 = match digits.parse() {
            Ok(i) => i,
            Err(err) => panic!("failed to parse digits: {} - {:?}", digits, err),
        };

        values.push(combined);
    }

    let total: u32 = values.iter().sum();

    println!("## TOTAL: {} ##", total);
}

fn get_last_digit(word_numbers: &Vec<String>, line: &String) -> char {
    if line.len() == 0 {
        return '0';
    }

    let mut idx_word = 0;
    let mut word: &String = &String::new();
    for n in 0..word_numbers.len() {
        let w = &word_numbers[n];
        let mut is_match = false;

        let idx = match line.rfind(&word_numbers[n]) {
            Some(i) => {
                is_match = true;
                i
            }
            None => 0,
        };

        if is_match && idx >= idx_word {
            idx_word = idx;
            word = w;
        }
    }

    let mut idx_char = line.len() - 1;
    let mut digit: char = '0';
    for c in line.chars().rev() {
        if c.is_digit(10) {
            digit = c;
            break;
        }

        if idx_char > 0 {
            idx_char -= 1;
        }
    }

    if !word.is_empty() && idx_word >= idx_char {
        return get_digit_from_str(word);
    } else {
        return digit;
    }
}

fn get_first_digit(word_numbers: &Vec<String>, line: &String) -> char {
    let mut idx_word = line.len();
    let mut word: &String = &String::new();
    for n in 0..word_numbers.len() {
        let w = &word_numbers[n];

        let idx = match line.find(&word_numbers[n]) {
            Some(i) => i,
            None => line.len(),
        };

        if idx < idx_word {
            idx_word = idx;
            word = w;
        }
    }

    let mut idx_char = 0;
    let mut digit: char = '0';
    for c in line.chars() {
        if c.is_digit(10) {
            digit = c;
            break;
        }

        idx_char += 1;
    }

    if idx_word == idx_char {
        return '0';
    }

    if idx_word < idx_char {
        return get_digit_from_str(word);
    } else {
        return digit;
    }
}

fn get_digit_from_str(word: &str) -> char {
    match word {
        "zero" => return '0',
        "one" => return '1',
        "two" => return '2',
        "three" => return '3',
        "four" => return '4',
        "five" => return '5',
        "six" => return '6',
        "seven" => return '7',
        "eight" => return '8',
        "nine" => return '9',
        _ => return '0',
    }
}
