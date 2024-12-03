fn after_keyword_indices(raw_chars: &[u8], keyword: &[u8]) -> Vec<usize> {
    let mut indices = vec![];

    for (index, window) in raw_chars.windows(keyword.len()).enumerate() {
        if window.eq(keyword) {
            indices.push(index + keyword.len());
        }
    }

    indices
}

fn index_of_closing_paren(raw_chars: &[u8]) -> Option<usize> {
    let needle = b')';

    for (index, c) in raw_chars.iter().enumerate() {
        if c == &needle {
            return Some(index);
        }
    }

    None
}

fn find_instruction_bodies(raw_chars: &[u8]) -> Vec<&[u8]> {
    let mut bounds = vec![];

    for start in after_keyword_indices(raw_chars, "mul(".as_bytes()) {
        let rest = &raw_chars[start..];

        if let Some(offset_end) = index_of_closing_paren(rest) {
            let end = offset_end + start;
            bounds.push(&raw_chars[start..end]);
        } else {
            return bounds;
        }
    }

    bounds
}

fn parse_instruction_body(body: &str) -> Option<(u32, u32)> {
    let (left, right) = body.split_once(',')?;

    Some((left.parse().ok()?, right.parse().ok()?))
}

pub fn part_1(input: &str) -> String {
    let mut sum = 0;

    for body in find_instruction_bodies(input.as_bytes()) {
        let body = String::from_utf8_lossy(body);
        let Some((left, right)) = parse_instruction_body(&body) else {
            continue;
        };

        sum += left * right;
    }

    sum.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut raw_chars_filtered = vec![];
    let adjusted = format!("do(){}", input);

    for part in adjusted.split("don't()") {
        let Some((_, enabled)) = part.split_once("do()") else {
            continue;
        };

        raw_chars_filtered.extend_from_slice(enabled.as_bytes());
    }

    let input = String::from_utf8_lossy(&raw_chars_filtered);

    part_1(&input)
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
