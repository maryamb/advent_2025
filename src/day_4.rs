use std::collections::VecDeque;


fn string_to_table(text: String) -> Vec<Vec<char>> {
    text
        .trim()
        .lines()
        .map(|line| line.chars().collect()) 
        .collect()
}

fn get_8_neighbors(i: usize, j: usize, rows: usize, cols: usize) -> ([(usize, usize); 8], usize) {
    let mut out = [(0, 0); 8];
    let mut count = 0;
    let indexes = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    for (di, dj) in indexes.iter() { 
        if let (Some(r), Some(c)) = (i.checked_add_signed(*di), j.checked_add_signed(*dj)) {
            if r < rows && c < cols {
                out[count] = (r, c);
                count += 1;
            }
        }
    }
    (out, count)
}

fn count_adjacents(input: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let rows = input.len();
    let cols = input[0].len();
    let mut out = vec![vec![0; cols]; rows];
    input.iter()
        .enumerate()
        .for_each(|(i, row)| {
            row.iter()
                .enumerate()
                .for_each(|(j, c)| {
                    let (neighbors, count) = get_8_neighbors(i, j, rows, cols);
                    let adjacent_rollers = neighbors.iter().take(count).filter(|(r, c)| input[*r][*c] == '@').count();
                    if *c == '@' && adjacent_rollers < 4 {
                        out[i][j] = 1;
                    }
                })
        });
    out
}

fn get_num_adjacent_roller_papers(input: String) -> usize {
    let table = string_to_table(input);
    let adjacents = count_adjacents(&table);
    adjacents.iter().flatten().sum()
}

fn iterate_until_unchanged(input: String) -> usize {
    let mut table = string_to_table(input);
    let mut total_removed_roller_papers = 0;
    loop {
        let adjacents = count_adjacents(&table);
        let mut changed = false;
        for i in 0..table.len() {
            for j in 0..table[0].len() {
                if adjacents[i][j] == 1 {
                    table[i][j] = '.';
                    total_removed_roller_papers += 1;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    total_removed_roller_papers
}

fn iterate_until_unchanged_optimized(input: String) -> usize {
    let mut table = string_to_table(input);
    let mut count = vec![vec![0; table[0].len()]; table.len()];
    for i in 0..table.len() {
        for j in 0..table[0].len() {
            if table[i][j] == '@' {
                let (neighbors, count_neighbors) = get_8_neighbors(i, j, table.len(), table[0].len());
                for k in 0..count_neighbors {
                    let (r, c) = neighbors[k];
                    count[r][c] += 1;
                }
            }
        }
    }

    let mut queue = VecDeque::new();
    for i in 0..table.len() {
        for j in 0..table[0].len() {
            if table[i][j] == '@' && count[i][j] < 4 {
                queue.push_back((i, j));
            }
        }
    }

    let mut removed = 0;
    while let Some((i, j)) = queue.pop_front() {
        if table[i][j] == '@' && count[i][j] < 4 {
            table[i][j] = '.';
            removed += 1;
            let (neighbors, count_neighbors) = get_8_neighbors(i, j, table.len(), table[0].len());
            for k in 0..count_neighbors {
                let (r, c) = neighbors[k];
                count[r][c] -= 1;
                if table[r][c] == '@' && count[r][c] == 3 {
                    queue.push_back((r, c));
                }
            }
        }
    }
    removed
}



#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const TEST_STR: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_string_to_table() {
        let table = string_to_table(TEST_STR.to_string());
        for row in &table {
            //println!("{}", row.iter().collect::<String>());
        }
        assert_eq!(table.len(), 10);
        assert_eq!(table[0].len(), 10);
        assert_eq!(table[0][2], '@');
        assert_eq!(table[9][8], '@');
    }

    #[test]
    fn test_count_adjacents() {
        let table = string_to_table(TEST_STR.to_string());
        let adjacents = count_adjacents(&table);
        for row in &adjacents {
            // println!("{:?}", row);
        }
        assert_eq!(adjacents.iter().flatten().sum::<usize>(), 13);
    }

    #[test]
    fn test_get_num_adjacent_roller_papers() {
        let num = get_num_adjacent_roller_papers(TEST_STR.to_string());
        assert_eq!(num, 13);
    }

    #[test]
    fn test_string_to_table_and_count_adjacents() {
        let input = std::fs::read_to_string("data/day_4.txt")
            .expect("Failed to read day_4_input.txt");

    let table = string_to_table(input);
    let adjacents = count_adjacents(&table);
    let total: usize = adjacents.iter().flatten().sum();

    println!("Accessible rolls: {total}");
        assert_eq!(adjacents.iter().flatten().sum::<usize>(), 1349);
    }

    #[test]
    fn test_iterate_until_unchanged() {
        let num = iterate_until_unchanged(TEST_STR.to_string());
        assert_eq!(num, 43);
    }

    #[test]fn test_iterate_until_unchanged_file() {
        let input = std::fs::read_to_string("data/day_4.txt")
            .expect("Failed to read day_4_input.txt");
        let input_1 = input.clone();
        let now = std::time::Instant::now();
        let num = iterate_until_unchanged(input);
        assert_eq!(num, 8277);
        println!("Time taken for iterate_until_unchanged: {:?}", now.elapsed());
        let now = std::time::Instant::now();
        let num_optimized = iterate_until_unchanged_optimized(input_1);
        assert_eq!(num_optimized, 8277);
        println!("Time taken for iterate_until_unchanged_optimized: {:?}", now.elapsed());
    }

}
