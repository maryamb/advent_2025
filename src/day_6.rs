use rayon::{prelude::*, vec};

fn parse_input_part_1(input_text: &str) -> (Vec<Vec<usize>>, Vec<char>) {
    let mut lines = input_text.lines();
    let signs_str = lines.next_back().expect("there should be signs line");
    let numbers: Vec<Vec<usize>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<usize>().expect("must be a number"))
                .collect()
        })
        .collect();

    let signs: Vec<char> = signs_str
        .split_whitespace()
        .map(|s| s.parse::<char>().expect("must be a char"))
        .collect();

    (transpose(numbers), signs)
}

fn transpose<T: Copy + Default>(input_v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows_count = input_v.len();
    let cols_count = input_v.first().map_or(0, Vec::len);

    let mut transposed = vec![vec![T::default(); rows_count]; cols_count];
    for i in 0..rows_count {
        for j in 0..cols_count {
            transposed[j][i] = input_v[i][j];
        }
    }
    transposed
}

fn process_column(col: &Vec<usize>, sign: char) -> usize {
    match sign {
        '+' => col.iter().fold(0, |a, b| a + b),
        '*' => col.iter().fold(1, |a, b| a * b),
        _ => 0,
    }
}

fn part_1(input_text: &str) -> usize {
    let (numbers, signs) = parse_input_part_1(input_text);
    numbers
        .iter()
        .zip(signs)
        .map(|(col, sign)| process_column(col, sign))
        .sum()
}

fn parse_input_part_2(input_text: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut lines = input_text.lines();
    let signs_str = lines.next_back().expect("there should be signs line");
    let numbers: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let signs: Vec<char> = signs_str
        .split_whitespace()
        .map(|s| s.parse::<char>().expect("must be a char"))
        .collect();

    let numbers = transpose(numbers);

    (numbers, signs)
}

fn get_numbers_from_all_chars_matrix(numbers: Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut current: usize = 0;
    let mut col_numbers: Vec<Vec<usize>> = vec![];
    let mut current_col: Vec<usize> = vec![];
    for row in numbers {
        let all_spaces = row.iter().all(|&c| c == ' ');
        if all_spaces {
            col_numbers.push(current_col);
            current_col = vec![];
            continue;
        }
        let string: String = row.into_iter().collect();
        let number = string.trim().parse::<usize>().expect("must be a number");
        current_col.push(number);
    }
    if !current_col.is_empty() {
        col_numbers.push(current_col);
    }

    col_numbers
}

fn part_2(input_text: &str) -> usize {
    let (numbers, signs) = parse_input_part_2(input_text);
    let numbers = get_numbers_from_all_chars_matrix(numbers);
    numbers
        .iter()
        .zip(signs)
        .map(|(col, sign)| process_column(col, sign))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const TEST_STR: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_parse_input() {
        let (columns, signs) = parse_input_part_1(TEST_STR);

        assert_eq!(
            columns,
            vec![
                vec![123, 45, 6],
                vec![328, 64, 98],
                vec![51, 387, 215],
                vec![64, 23, 314],
            ]
        );
        assert_eq!(signs, vec!['*', '+', '*', '+']);
    }

    #[test]
    fn test_parse_input_part_2() {
        let (columns, signs) = parse_input_part_2(TEST_STR);
        println!("numbers: {:?}\nsigns: {:?}\n\n", columns, signs);
        println!("{:?}", get_numbers_from_all_chars_matrix(columns));

        assert_eq!(part_2(TEST_STR), 3_263_827);
    }

    #[test]
    fn test_transpose() {
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];

        assert_eq!(transpose(rows), vec![vec![1, 4], vec![2, 5], vec![3, 6],]);
    }

    #[test]
    fn test_process_column() {
        assert_eq!(process_column(&vec![2, 3, 4], '+'), 9);
        assert_eq!(process_column(&vec![2, 3, 4], '*'), 24);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_STR), 4_277_556);
    }

    #[test]
    fn test_part_1_file_input() {
        let input_text =
            std::fs::read_to_string("data/day_6.txt").expect("Failed to read day_6_input.txt");
        assert_eq!(part_1(input_text.as_str()), 5_316_572_080_628);
        assert_eq!(part_2(input_text.as_str()), 11_299_263_623_062);
    }
}
