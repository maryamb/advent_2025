
fn get_invalids(lower: i64, upper: i64) -> i64 {
    (lower..=upper)
    .map(|num| {
        let num_str = num.to_string();
        let mid = num_str.len() / 2;
        if num_str.len() % 2 != 0 {
            return 0;
        }
        let (left, right) = num_str.split_at(mid);
        if left == right {
            return num;
        }
        0
    })
    .sum()
}

fn get_invalids_v2(lower: i64, upper: i64) -> i64 {
    (lower..=upper)
    .filter(|num| {
        let num_str = num.to_string();
        (1..=num_str.len() / 2)
        .any(|current_len| {
            if num_str.len() % current_len != 0 {
                return false;
            }
            
            let pattern = &num_str[0..current_len];
            num_str.as_bytes()
                    .chunks(current_len)
                    .all(|chunk| std::str::from_utf8(chunk).unwrap() == pattern)
        })
    })
    .sum()
}



fn day_2_1(all_ranges: &str, get_invalids: fn(i64, i64) -> i64) -> i64 {
    all_ranges
    .split(',')
    .map(|s| {
        let (left, right) = s.split_once('-').expect("msg");
        let left: i64 = left.parse().expect("msg");
        let right: i64 = right.parse().expect("msg");
        (left, right)
    })
    .fold(0, |acc, (a, b)| acc + get_invalids(a, b))
}


#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(get_invalids(11, 22), 33);
        let all_ranges = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                                1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                                824824821-824824827,2121212118-2121212124";
        assert_eq!(day_2_1(all_ranges, get_invalids), 1227775554);
        assert_eq!(day_2_1(all_ranges, get_invalids_v2), 4174379265);
    }

    #[test]
    fn test_example_2() {
        let all_ranges = fs::read_to_string("data/day_2.txt").expect("msg");
        assert_eq!(day_2_1(&all_ranges, get_invalids), 38437576669);
        assert_eq!(day_2_1(&all_ranges, get_invalids_v2), 49046150754);
    }
}