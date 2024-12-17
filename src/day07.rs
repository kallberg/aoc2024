use rayon::{iter::ParallelIterator, str::ParallelString};

#[derive(Debug)]
struct Equation {
    test: usize,
    numbers: Vec<usize>,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (test_str, rest) = value.split_once(':').unwrap();
        let test: usize = test_str.parse().unwrap();
        let numbers = rest
            .split_whitespace()
            .flat_map(|value| value.parse())
            .collect();

        Self { test, numbers }
    }
}

impl Equation {
    fn valid(&self, include_concat: bool) -> bool {
        let mut count = 0;
        matches(&self.numbers, self.test, include_concat, &mut count);
        count > 0
    }
}

fn concat(lhs: usize, rhs: usize) -> usize {
    let mag = (rhs as f64).log10().floor() as usize + 1;
    lhs * 10_usize.pow(mag as u32) + rhs
}

fn matches(numbers: &[usize], test: usize, include_concat: bool, count: &mut usize) {
    if numbers.is_empty() {
        return;
    }

    if numbers.len() == 1 {
        if numbers[0] == test {
            *count += 1;
        }
        return;
    }

    let lhs = numbers[0];
    let rhs = numbers[1];
    let add = lhs + rhs;
    let mul = lhs * rhs;

    if add <= test {
        let add_branch: Vec<usize> = std::iter::once(add)
            .chain(numbers[2..].iter().copied())
            .collect();
        matches(&add_branch, test, include_concat, count);
    }
    if mul <= test {
        let mul_branch: Vec<usize> = std::iter::once(mul)
            .chain(numbers[2..].iter().copied())
            .collect();
        matches(&mul_branch, test, include_concat, count);
    }

    if include_concat {
        let con = concat(lhs, rhs);

        if con <= test {
            let con_branch: Vec<usize> = std::iter::once(con)
                .chain(numbers[2..].iter().copied())
                .collect();

            matches(&con_branch, test, include_concat, count);
        }
    }
}

pub fn part_1(input: &str) -> String {
    let sum: usize = input
        .par_lines()
        .map(|line| {
            let equation = Equation::from(line);
            if equation.valid(false) {
                return equation.test;
            }

            0
        })
        .sum();

    sum.to_string()
}
pub fn part_2(input: &str) -> String {
    let sum: usize = input
        .par_lines()
        .map(|line| {
            let equation = Equation::from(line);
            if equation.valid(true) {
                return equation.test;
            }

            0
        })
        .sum();

    sum.to_string()
}

#[cfg(test)]
mod test {
    use crate::{
        day07::{concat, part_1, part_2},
        input, output,
    };

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(input::example::DAY_07), output::example::DAY_07_1);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(input::DAY_07), output::DAY_07_1)
    }

    #[test]
    fn concat_test() {
        assert_eq!(12345usize, concat(12, 345));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(input::example::DAY_07), output::example::DAY_07_2);
    }
    #[test]
    fn part_2_real() {
        assert_eq!(part_2(input::DAY_07), output::DAY_07_2)
    }
}
