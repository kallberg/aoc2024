fn diag_down(x: usize, y: usize, height: usize) -> usize {
    x + height - y
}

fn diag_up(x: usize, y: usize) -> usize {
    x + y
}

fn count_xmas(value: &str) -> usize {
    let mut xmas = 0;

    for window in value.as_bytes().windows(4) {
        if window == "XMAS".as_bytes() {
            xmas += 1;
        }
        if window == "SAMX".as_bytes() {
            xmas += 1;
        }
    }

    xmas
}

fn count_many(values: &Vec<String>) -> usize {
    let mut xmas = 0;

    for value in values {
        xmas += count_xmas(value)
    }

    xmas
}

pub fn part_1(input: &str) -> String {
    type Strings = Vec<String>;

    let rows: Strings = input.lines().map(|line| line.to_owned()).collect();
    let mut columns: Strings = vec![];
    let mut downward: Strings = vec![];
    let mut upward: Strings = vec![];

    for (y, line) in rows.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            let d = diag_down(x, y, rows.len());
            let u = diag_up(x, y);

            while x >= columns.len() {
                columns.push(String::new())
            }

            while d >= downward.len() {
                downward.push(String::new());
            }

            while u >= upward.len() {
                upward.push(String::new());
            }

            columns[x].push(character);
            downward[d].push(character);
            upward[u].push(character);
        }
    }

    let mut xmas = 0;

    xmas += count_many(&rows);
    xmas += count_many(&columns);
    xmas += count_many(&downward);
    xmas += count_many(&upward);

    xmas.to_string()
}

fn outer_match(first: u8, second: u8) -> bool {
    if first == second {
        return false;
    }

    if first != b'M' && first != b'S' {
        return false;
    }

    if second != b'M' && second != b'S' {
        return false;
    }

    true
}

pub fn part_2(input: &str) -> String {
    type Strings = Vec<String>;

    let rows: Strings = input.lines().map(|line| line.to_owned()).collect();
    let mut x_mas = 0;

    for (y, row) in rows.iter().enumerate() {
        for (x, char) in row.chars().enumerate() {
            if y == 0 || y + 1 == rows.len() {
                continue;
            }
            if x == 0 || x + 1 == row.len() {
                continue;
            }

            if char != 'A' {
                continue;
            }

            let prev_row = &rows[y - 1];
            let next_row = &rows[y + 1];

            let top_left = prev_row.as_bytes()[x - 1];
            let bot_down = next_row.as_bytes()[x + 1];
            let bot_left = next_row.as_bytes()[x - 1];
            let top_right = prev_row.as_bytes()[x + 1];

            if !outer_match(top_left, bot_down) {
                continue;
            }

            if !outer_match(bot_left, top_right) {
                continue;
            }

            x_mas += 1;
        }
    }

    x_mas.to_string()
}

#[cfg(test)]
mod test {
    use crate::{
        day04::{part_1, part_2},
        input, output,
    };

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(input::example::DAY_04), output::example::DAY_04_1);
    }
    #[test]
    fn part_1_real() {
        assert_eq!(part_1(input::DAY_04), output::DAY_04_1)
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(input::example::DAY_04), output::example::DAY_04_2);
    }
    #[test]
    fn part_2_real() {
        assert_eq!(part_2(input::DAY_04), output::DAY_04_2)
    }
}
