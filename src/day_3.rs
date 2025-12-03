fn get_bank_joltage(bank: &str) -> u64 {
    let bank: Vec<u64> = bank.chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    let (max_i, max_v) = bank.iter().enumerate()
        .max_by(|a, b| a.1.cmp(b.1).then(std::cmp::Ordering::Less))
        .map(|(i, v)| (i, v))
        .expect("msg");
    let (l, r) = match max_i < bank.len() - 1 {
        true => (1 + max_i, bank.len()),
        false => (0, max_i)
    };
    let (max_j, max_w) = bank[l..r].iter().enumerate()
        .max_by(|a, b| a.1.cmp(b.1).then(std::cmp::Ordering::Less))
        .map(|(i, v)| (i + l, v))
        .expect("msg");
    match max_i < max_j {
        true => max_v * 10 + max_w,
        false => max_w * 10 + max_v
    }
}

fn adjust_opt(opt_joltage: &mut Vec<u64>, index: usize, new_joltage: u64) {
    if index >= opt_joltage.len() || new_joltage < opt_joltage[index] { return; }
    let candidate_joltage = opt_joltage[index];
    opt_joltage[index] = new_joltage;
    adjust_opt(opt_joltage, index + 1, candidate_joltage);
}

fn dozen_joltage(bank: &str) -> u64 {
    let bank: Vec<u64> = bank.chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    let start = bank.len().saturating_sub(12);
    let mut opt_joltage = bank[start..].to_vec();
    bank[0..start].iter().rev()
        .for_each(|joltage| {
            adjust_opt(&mut opt_joltage, 0, *joltage);
            // println!("opt_joltage: {:?}", opt_joltage);
        });
    opt_joltage.iter().fold(0, |acc, item| acc * 10 + item)
}

fn all_banks_joltage(all_banks: &String) -> u64 {
        all_banks.lines()
            .map(|bank| get_bank_joltage(bank))
            .sum()
}

fn all_banks_dozen_joltage(all_banks: &String) -> u64 {
        all_banks.lines()
            .map(|bank| dozen_joltage(bank))
            .sum()
}


#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_example_3() {
        // assert_eq!(dozen_joltage("987654321111111"), 987654321111);
        // assert_eq!(dozen_joltage("811111111111119"), 811111111119);
        // assert_eq!(dozen_joltage("234234234234278"), 434234234278);
        assert_eq!(dozen_joltage("818181911112111"), 888911112111);
    }

    #[test]
    fn test_example_1() {
        assert_eq!(get_bank_joltage("987654321111111"), 98);
        assert_eq!(get_bank_joltage("811111111111119"), 89);
        assert_eq!(get_bank_joltage("234234234234278"), 78);
        assert_eq!(get_bank_joltage("6317622422223322166413226232114422344333123467172215425555122323342222642724762452622734122423214722"), 99);
        assert_eq!(get_bank_joltage("818181911112111"), 92);
    }

    #[test]
    fn test_example_2() {
        let all_banks = fs::read_to_string("data/day_3.txt").expect("msg");
        assert_eq!(all_banks_joltage(&all_banks), 16854);
    }
    
    #[test]
    fn test_example_4() {
        let all_banks = fs::read_to_string("data/day_3.txt").expect("msg");
        assert_eq!(all_banks_dozen_joltage(&all_banks), 16854);
    }
}