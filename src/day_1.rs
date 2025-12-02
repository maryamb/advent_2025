
fn day_1_1(lines: &Vec<&str>) -> usize {
    lines
    .iter()
    .scan(50 as i32, |state, current_line| {
        println!("{}", state);
        let dial: i32 = current_line[1..].parse().expect("");
        *state = match current_line.starts_with("L") {
            true => *state - dial,
            _ => *state + dial,
        };
        *state = *state % 100;
        Some(*state)
    })
    .filter(|state| *state == 0)
    .count()
}

fn day_1_2(lines: &Vec<&str>) -> i32 {
    lines
    .iter()
    .scan(50 as i32, |state, current_line| {
        let last_state = *state;
        let dial: i32 = current_line[1..].parse().expect("");
        let dial = match current_line.starts_with("L") {
            true => -dial,
            _ => dial,
        };
        let dir = dial % 100;
        *state = (*state + dial) % 100;
        let mut passed_zero = 0;
        if *state > 0 && last_state < 0 || *state < 0 && last_state > 0 {
            passed_zero += 1;
        }
        let wrapes = dial.abs() / 100;
        passed_zero += wrapes;
        if *state == 0 && dir != 0 {
            passed_zero += 1;
        }
        
        Some(passed_zero)
    })
    .sum()
}

fn day_1_2_bf(lines: &Vec<&str>) -> i32 {
    lines
    .iter()
    .scan(50 as i32, |state, current_line| {
        let dial: i32 = current_line[1..].parse().expect("");
        let delta = match current_line.starts_with("L") {
            true => -1,
            _ => 1,
        };
        let mut zero_pass = 0;
        for _  in 0..dial {
            *state = (*state + delta).rem_euclid(100);
            if *state == 0 {
                zero_pass += 1;
            }
        }
        Some(zero_pass)
        
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_example_1() {
        let lines = vec!["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"];
        // let lines = vec!["R50"];
        // assert_eq!(day_1_1(&lines), 3);
        assert_eq!(day_1_2(&lines), 6);
    }
    #[test]
    fn test_example_2() {
        let binding = fs::read_to_string("data/day_1.txt").expect("msg");
        let lines = binding
        .lines()
        .collect();
        // assert_eq!(964, day_1_1(&lines));
        println!("{}", day_1_2(&lines));
        assert_eq!(5872, day_1_2_bf(&lines));
    }
}