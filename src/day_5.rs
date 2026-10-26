use rayon::prelude::*;


fn sort_and_merge_intervals(mut intervals: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut output_intervals: Vec<(usize, usize)> = vec![];
    intervals.sort();
    let mut current = intervals[0];
    for interval in intervals.iter().skip(1) {
        if interval.0 > current.1 {
            output_intervals.push(current.clone());
            current = *interval;
        } else if interval.1 > current.1 {
            current.1 = interval.1
        }
    }
    output_intervals.push(current);
    output_intervals
}

fn contains(intervals: &[(usize, usize)], target: usize) -> bool {
    let index = intervals.partition_point(|&(start, _)| start <= target);
    index > 0 && target <= intervals[index - 1].1
}

fn part_1(input_text: String) -> usize {
    let (intervals, values) = parse_input(input_text);
    let intervals = sort_and_merge_intervals(intervals);
    values
    .iter()
    .filter(|value| contains(&intervals, **value))
    .count()
}

fn part_2(input_text: String) -> usize {
    let (intervals, _) = parse_input(input_text);
    let intervals = sort_and_merge_intervals(intervals);

    intervals
    .iter()
    .map(|(l, r)| r - l + 1)
    .sum()
}

fn part_1_par(input_text: String) -> usize {
    let (intervals, values) = parse_input(input_text);
    let intervals = sort_and_merge_intervals(intervals);

    values
        .par_iter()
        .filter(|&&value| contains(&intervals, value))
        .count()
}

fn parse_input(input_text: String) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (intervals, values) = input_text
        .split_once("\n\n")
        .expect("Expected a blank line");
    let intervals: Vec<(usize, usize)> = intervals
    .lines()
    .map(|s| {
        let (l, r) = s.split_once("-").expect("expected l-r");
        (l.parse::<usize>().expect("invalid left endpoint"),
        r.parse::<usize>().expect("invalid right endpoint"))
    })
    .collect();

    let values: Vec<usize> = values
        .lines()
        .map(|s| s.parse::<usize>().expect("invalid value"))
        .collect();

    (intervals, values)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const TEST_STR: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_sort_and_merge_intervals() {
        let intervals = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        assert_eq!(sort_and_merge_intervals(intervals), vec![(3, 5), (10, 20)]);
    }

    #[test]
    fn tes_parse_input() {
        let (intervals, values) = parse_input(TEST_STR.to_string());
        println!("intervals: {:?} \nvalues: {:?}\n", intervals, values);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(3,  part_1(TEST_STR.to_string()));
        let input = std::fs::read_to_string("data/day_5.txt")
            .expect("Failed to read day_5_input.txt");
        let input_1 = input.clone();
        let now = std::time::Instant::now();
        assert_eq!(513,  part_1(input));
        println!("Time taken for normal part_1: {:?}", now.elapsed());
        let now = std::time::Instant::now();
        assert_eq!(513,  part_1_par(input_1));
        println!("Time taken for parallel part_1: {:?}", now.elapsed());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(14, part_2(TEST_STR.to_string()));
        let input = std::fs::read_to_string("data/day_5.txt")
            .expect("Failed to read day_5_input.txt");
        assert_eq!(339668510830757,  part_2(input));
    }

}
