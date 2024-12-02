#[derive(PartialEq, Eq, Clone)]
enum Change {
    Increase,
    Decrease,
    Invalid,
    None,
}

impl Change {
    fn from_left_right(left: u32, right: u32) -> Self {
        if left == right {
            return Change::Invalid;
        }
        if left > right {
            return Change::Increase;
        }
        Change::Decrease
    }

    fn safe(&mut self, left: u32, right: u32) -> bool {
        if self == &Change::Invalid {
            return false;
        }

        let next = Change::from_left_right(left, right);

        if self == &Change::None {
            *self = Change::from_left_right(left, right);
        }

        if self != &Change::from_left_right(left, right) {
            return false;
        }

        let delta = left.abs_diff(right);

        if !(1..=3).contains(&delta) {
            return false;
        }

        *self = next;

        true
    }

    fn safe_report(mut self, report: &[u32], dampener_available: bool) -> bool {
        for window in report.windows(2) {
            if !self.safe(window[0], window[1]) {
                if !dampener_available {
                    return false;
                } else {
                    for variant_index in 0..report.len() {
                        let mut variant = vec![];
                        for (index, entry) in report.iter().enumerate() {
                            if index == variant_index {
                                continue;
                            }

                            variant.push(*entry);
                        }

                        if Change::None.safe_report(&variant, false) {
                            return true;
                        }
                    }

                    return false;
                }
            }
        }

        true
    }
}

#[allow(dead_code)]
pub fn part_1(input: &str) -> String {
    let mut safe = 0;

    for line in input.lines() {
        let change = Change::None;
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();

        if change.safe_report(&report, false) {
            safe += 1;
        }
    }

    safe.to_string()
}

#[allow(dead_code)]
pub fn part_2(input: &str) -> String {
    let mut safe = 0;

    for line in input.lines() {
        let change = Change::None;
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();

        if change.safe_report(&report, true) {
            safe += 1;
        }
    }

    safe.to_string()
}

#[cfg(test)]
mod test {
    use crate::{
        day02::{part_1, part_2},
        input, output,
    };

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(input::example::DAY_02), output::example::DAY_02_1);
    }
    #[test]
    fn part_1_real() {
        assert_eq!(part_1(input::DAY_02), output::DAY_02_1);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(input::example::DAY_02), output::example::DAY_02_2);
    }
    #[test]
    fn part_2_real() {
        assert_eq!(part_2(input::DAY_02), output::DAY_02_2)
    }
}
