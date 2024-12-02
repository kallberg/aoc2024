use std::collections::HashMap;

pub fn part_1(input: &str) -> String {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in input.lines() {
        let mut split = line.split_whitespace();

        let left_str = split.next().unwrap();
        let right_str = split.next().unwrap();

        left.push(left_str.parse().unwrap());
        right.push(right_str.parse().unwrap());
    }

    left.sort();
    right.sort();

    let mut sum = 0;

    for (left, right) in left.into_iter().zip(right.into_iter()) {
        sum += left.abs_diff(right);
    }

    sum.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut right_freq: HashMap<u32, u32> = HashMap::new();
    let mut left: Vec<u32> = vec![];

    for line in input.lines() {
        let mut split = line.split_whitespace();

        let left_value: u32 = split.next().unwrap().parse().unwrap();
        let right_value: u32 = split.next().unwrap().parse().unwrap();

        left.push(left_value);

        if let Some(entry) = right_freq.get_mut(&right_value) {
            *entry += 1;
        } else {
            right_freq.insert(right_value, 1);
        }
    }

    let mut sum = 0;

    for entry in left {
        let freq = *(right_freq.get(&entry).unwrap_or(&0));
        sum += entry * freq;
    }

    sum.to_string()
}

#[cfg(test)]
mod test {
    use crate::{
        day01::{part_1, part_2},
        input, output,
    };

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(input::example::DAY_01), output::example::DAY_01_1);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(input::DAY_01), output::DAY_01_1);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(input::example::DAY_01), output::example::DAY_01_2);
    }
    #[test]
    fn part_2_real() {
        assert_eq!(part_2(input::DAY_01), output::DAY_01_2)
    }
}
