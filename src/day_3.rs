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
    // println!("l: {l}, r: {r}");
    let (max_j, max_w) = bank[l..r].iter().enumerate()
        .max_by(|a, b| a.1.cmp(b.1).then(std::cmp::Ordering::Less))
        .map(|(i, v)| (i + l, v))
        .expect("msg");
    // println!("max_i: {max_i}, max_v: {max_v}, max_j: {max_j}, max_w: {max_w}");
    match max_i < max_j {
        true => max_v * 10 + max_w,
        false => max_w * 10 + max_v
    }
}

fn all_banks_joltage(all_banks: &String) -> u64 {
        all_banks.lines()
            .map(|bank| get_bank_joltage(bank))
            .sum()
}


#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

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
}