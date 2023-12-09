// Note: There must be a mathematical equation to prevent looping through every results
// Couldn't figure it out :(

struct TimeDistance {
    time: Vec<usize>,
    distance: Vec<usize>,
}

fn main() {
    let data = util::get_data("data.txt");

    println!("## part 1: {} ##", part1_solution(&data));
    println!("## part 2: {} ##", part2_solution(&data));
}

fn part2_solution(d: &Vec<String>) -> usize {
    let data = format_data_part2(d);

    let mut counts: Vec<usize> = Vec::new();

    for i in 0..data.time.len() {
        let t = data.time[i];
        let d = data.distance[i];

        let mut count = 0;

        for j in 0..t {
            if j * (t - j) > d {
                count += 1;
            }
        }

        counts.push(count);
    }

    return counts.iter().product();
}

fn part1_solution(d: &Vec<String>) -> usize {
    let data = format_data_part1(d);

    let mut counts: Vec<usize> = Vec::new();

    for i in 0..data.time.len() {
        let t = data.time[i];
        let d = data.distance[i];

        let mut count = 0;

        for j in 0..t {
            if j * (t - j) > d {
                count += 1;
            }
        }

        counts.push(count);
    }

    return counts.iter().product();
}

fn format_data_part1(d: &Vec<String>) -> TimeDistance {
    let times = d[0].split(":").collect::<Vec<&str>>()[1];
    let distances = d[1].split(":").collect::<Vec<&str>>()[1];

    return TimeDistance {
        time: util::get_numbers_from_str(times, " "),
        distance: util::get_numbers_from_str(distances, " "),
    };
}

fn format_data_part2(d: &Vec<String>) -> TimeDistance {
    let time = d[0].split(":").collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<usize>()
        .expect("failed to parse time");
    let distance = d[1].split(":").collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<usize>()
        .expect("failed to parse distance");

    return TimeDistance {
        time: vec![(time)],
        distance: vec![(distance)],
    };
}
