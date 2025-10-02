// https://adventofcode.com/2024/day/1

pub fn solve(input: String) -> (String, String) {
    let lines = input.lines();
    let location_ids = lines.map(|line| {
        let mut split = line.split_whitespace();
        let left_id = split.next().unwrap().parse::<i32>().unwrap();
        let right_id = split.next().unwrap().parse::<i32>().unwrap();
        (left_id, right_id)
    });

    let (mut left_ids, mut right_ids): (Vec<_>, Vec<_>) = location_ids.unzip();
    left_ids.sort();
    right_ids.sort();

    let mut sum = 0;
    for i in 0..left_ids.len() {
        sum += (left_ids[i] - right_ids[i]).abs()
    }

    // Part 2
    let mut similarity = 0;
    for i in 0..left_ids.len() {
        let mut right_count = 0;
        for id in right_ids.iter() {
            if *id == left_ids[i] {
                right_count += 1;
            }
        }
        similarity += left_ids[i] * right_count;
    }

    // Return the solution to part1 and part2 in a tuple.
    (sum.to_string(), similarity.to_string())
}
