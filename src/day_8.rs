use itertools::Itertools;
use std::collections::HashMap;

fn read_to_table(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u64>().expect("msg"))
                .collect()
        })
        .collect()
}

fn get_distance(table: &Vec<Vec<u64>>, i: usize, j: usize) -> u64 {
    let a = table[i][0].abs_diff(table[j][0]);
    let b = table[i][1].abs_diff(table[j][1]);
    let c = table[i][2].abs_diff(table[j][2]);
    a * a + b * b + c * c
}

fn get_all_distances(table: &Vec<Vec<u64>>) -> Vec<(u64, usize, usize)> {
    let rows = table.len();
    let mut distances: Vec<(u64, usize, usize)> = vec![];
    for i in 0..rows {
        for j in i + 1..rows {
            distances.push((get_distance(table, i, j), i, j));
        }
    }
    distances.sort();
    distances
}

fn find(parent: &HashMap<usize, usize>, i: usize, j: usize) -> (usize, usize) {
    let mut i_root = i;
    while parent[&i_root] != i_root {
        i_root = parent[&i_root];
    }

    let mut j_root = j;
    while parent[&j_root] != j_root {
        j_root = parent[&j_root];
    }

    (i_root, j_root)
}

fn union_find(distances: &Vec<(u64, usize, usize)>, max_count: usize) -> Vec<u64> {
    let mut parent: HashMap<usize, usize> = HashMap::new();
    let indexes: Vec<usize> = distances
        .iter()
        .map(|(_, i, j)| (i, j))
        .into_iter()
        .flat_map(|(a, b)| [*a, *b])
        .unique()
        .collect();
    for index in indexes {
        parent.insert(index, index);
    }

    for count in 0..max_count {
        let (_, i, j) = distances[count];
        let (i_root, j_root) = find(&parent, i, j);
        let root = i_root.min(j_root);

        for (_, v) in parent.iter_mut() {
            if *v == i_root || *v == j_root {
                *v = root;
            }
        }
    }
    let mut frequencies = HashMap::new();
    for value in parent.values() {
        // The entry API inserts 0 if the key doesn't exist, then increments it
        *frequencies.entry(value).or_insert(0) += 1;
    }
    frequencies.values().map(|&v| v as u64).collect()
}

fn union_find_with_cc(distances: &Vec<(u64, usize, usize)>) -> (usize, usize) {
    let mut parent: HashMap<usize, usize> = HashMap::new();
    let indexes: Vec<usize> = distances
        .iter()
        .map(|(_, i, j)| (i, j))
        .into_iter()
        .flat_map(|(a, b)| [*a, *b])
        .unique()
        .collect();
    for index in indexes.iter() {
        parent.insert(*index, *index);
    }
    let mut cc_count: usize = indexes.len();
    let mut count = 0;

    let (last_i, last_j) = loop {
        let (_, i, j) = distances[count];
        count += 1;
        let (i_root, j_root) = find(&parent, i, j);
        if i_root != j_root {
            let root = i_root.min(j_root);
            cc_count -= 1;
            for (_, v) in parent.iter_mut() {
                if *v == i_root || *v == j_root {
                    *v = root;
                }
            }
            if (cc_count == 1) {
                break (i, j);
            }
        }
    };

    (last_i, last_j)
}

fn part_1(table: &Vec<Vec<u64>>, max_count: usize) -> u64 {
    let distances = get_all_distances(table);
    let frequencies = union_find(&distances, max_count);
    frequencies
        .iter()
        .sorted_unstable_by_key(|v| std::cmp::Reverse(**v))
        .take(3)
        .product()
}

fn part_2(table: &Vec<Vec<u64>>) -> u64 {
    let distances = get_all_distances(table);
    let (last_i, last_j) = union_find_with_cc(&distances);
    table[last_i][0] * table[last_j][0]
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const TEST_STR: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_part_1_2_example() {
        let table = read_to_table(TEST_STR);
        let a = part_1(&table, 10);
        assert_eq!(a, 40);
        let b = part_2(&table);
        assert_eq!(b, 25272);
    }

    #[test]
    fn test_part_1_2_file_input() {
        let input_text =
            std::fs::read_to_string("data/day_8.txt").expect("Failed to read day_7_input.txt");
        let table = read_to_table(&input_text);
        let a = part_1(&table, 1000);
        assert_eq!(a, 80446);
        let a = part_2(&table);
        assert_eq!(a, 51294528);
    }
}
