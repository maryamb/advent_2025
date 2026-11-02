fn read_to_table(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn part_1(input: &str) -> usize {
    let table = read_to_table(input);

    let rows = table.len();
    let cols = table[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    let s_location = table[0]
        .iter()
        .position(|x| *x == 'S')
        .expect("There is S in the first row");

    let mut ray_queue: std::collections::VecDeque<(usize, usize)> =
        std::collections::VecDeque::new();
    ray_queue.push_back((0, s_location));
    let mut count: usize = 0;

    while let Some((i, j)) = ray_queue.pop_front() {
        if i >= rows || j >= cols || visited[i][j] {
            continue;
        }
        visited[i][j] = true;
        if table[i][j] == '^' {
            count += 1;
            if j > 0 {
                ray_queue.push_back((i, j - 1));
            }
            ray_queue.push_back((i, j + 1));
        } else {
            ray_queue.push_back((i + 1, j));
        }
    }

    count
}

fn part_2(input: &str) -> usize {
    let table = read_to_table(input);

    let rows = table.len();
    let cols = table[0].len();
    let mut paths: Vec<Vec<usize>> = vec![vec![0; cols]; rows];

    let s_location = table[0]
        .iter()
        .position(|x| *x == 'S')
        .expect("There is S in the first row");

    paths[1][s_location] = 1;

    for row in 1..rows {
        for col in 0..cols {
            if table[row - 1][col] == '.' {
                paths[row][col] += paths[row - 1][col];
            }
            if col > 0 && table[row - 1][col - 1] == '^' {
                paths[row][col] += paths[row - 1][col - 1];
            }
            if col + 1 < cols && table[row - 1][col + 1] == '^' {
                paths[row][col] += paths[row - 1][col + 1];
            }
        }
    }
    paths[rows - 1].iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const TEST_STR: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part_1_example() {
        let a = part_1(TEST_STR);
        assert_eq!(a, 21);
    }

    #[test]
    fn test_part_2_example() {
        let a = part_2(TEST_STR);
        assert_eq!(a, 40);
    }

    #[test]
    fn test_part_1_2_file_input() {
        let input_text =
            std::fs::read_to_string("data/day_7.txt").expect("Failed to read day_7_input.txt");
        let a = part_1(&input_text.as_str());
        assert_eq!(a, 1539);
        let a = part_2(&input_text.as_str());
        assert_eq!(a, 6479180385864);
    }
}
