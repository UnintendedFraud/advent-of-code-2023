use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum HandType {
    FiveKind = 7,
    FourKind = 6,
    FullHouse = 5,
    ThreeKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    t: HandType,
}

fn main() {
    let cards: HashMap<&str, usize> = HashMap::from([
        ("A", 13),
        ("K", 12),
        ("Q", 11),
        ("T", 10),
        ("9", 9),
        ("8", 8),
        ("7", 7),
        ("6", 6),
        ("5", 5),
        ("4", 4),
        ("3", 3),
        ("2", 2),
        ("J", 1),
    ]);

    let mut hands = format_data(&util::get_data("data.txt"));

    hands.sort_by(|a, b| sort_hands(a, b, &cards));

    let mut total = 0;

    for i in 0..hands.len() {
        total += hands[i].bid * (i + 1);
    }

    println!("## Total: {} ##", total);
}

fn sort_hands(h1: &Hand, h2: &Hand, cards: &HashMap<&str, usize>) -> Ordering {
    if h1.t > h2.t {
        return Ordering::Greater;
    } else if h1.t < h2.t {
        return Ordering::Less;
    }

    for i in 0..h1.cards.len() {
        let a = cards.get(h1.cards[i].to_string().as_str()).unwrap();
        let b = cards.get(h2.cards[i].to_string().as_str()).unwrap();

        if a > b {
            return Ordering::Greater;
        } else if a < b {
            return Ordering::Less;
        }
    }

    return Ordering::Equal;
}

fn format_data(data: &Vec<String>) -> Vec<Hand> {
    let mut d = Vec::new();

    for i in 0..data.len() {
        let split: Vec<&str> = data[i].split(" ").collect();

        let cards = String::from(split[0]).chars().collect();
        let bid = String::from(split[1]).parse().expect("failed to parse bid");
        let t = get_hand_type(&cards);

        d.push(Hand { cards, bid, t })
    }

    return d;
}

fn get_hand_type(chars: &Vec<char>) -> HandType {
    let mut cards_count: HashMap<String, usize> = HashMap::new();

    for i in 0..chars.len() {
        cards_count
            .entry(String::from(chars[i]))
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    let mut len = cards_count.len();

    let jeez = match cards_count.get("J") {
        Some(n) => *n,
        None => 0,
    };

    if len > 1 && jeez > 0 {
        len -= 1;
    }

    if len == 1 {
        return HandType::FiveKind;
    }
    if len == 2 {
        let max = get_max_non_jeez(&cards_count);
        if max + jeez == 4 {
            return HandType::FourKind;
        } else {
            return HandType::FullHouse;
        }
    }
    if len == 3 {
        let max = get_max_non_jeez(&cards_count);
        if max + jeez == 3 {
            return HandType::ThreeKind;
        } else {
            return HandType::TwoPair;
        }
    }
    if len == 4 {
        return HandType::OnePair;
    }

    return HandType::HighCard;
}

fn get_max_non_jeez(hash: &HashMap<String, usize>) -> usize {
    let mut max = 0;

    for (k, v) in hash {
        if k != "J" {
            if *v > max {
                max = *v;
            }
        }
    }

    return max;
}
