#[allow(dead_code)]
pub fn part_1(_input: &str) -> String {
    String::new()
}

#[allow(dead_code)]
pub fn part_2(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use crate::{
        day03::{part_1, part_2},
        input, output,
    };

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(input::example::DAY_03), output::example::DAY_03_1);
    }
    #[test]
    fn part_1_real() {
        assert_eq!(part_1(input::DAY_03), output::DAY_03_1)
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(input::example::DAY_03), output::example::DAY_03_2);
    }
    #[test]
    fn part_2_real() {
        assert_eq!(part_2(input::DAY_03), output::DAY_03_2)
    }
}
