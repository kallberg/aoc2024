use std::{collections::HashSet, fmt::Display, isize};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn dist(self, other: Position) -> (isize, isize) {
        let x = self.x as isize - other.x as isize;
        let y = self.y as isize - other.y as isize;

        (x, y)
    }
}

type Antenna = (Position, char);

struct Map {
    width: usize,
    height: usize,
    antennas: Vec<Antenna>,
    antinodes: HashSet<Position>,
}

impl Map {
    fn add_antinodes(&mut self, first: Position, second: Position, equal_dist: bool) {
        let (dx, dy) = first.dist(second);

        let mut iteration = 1;

        loop {
            let fx = first.x as isize + dx * iteration;
            let fy = first.y as isize + dy * iteration;
            let sx = second.x as isize - dx * iteration;
            let sy = second.y as isize - dy * iteration;

            let first_within_bounds =
                fx >= 0 && fx < self.width as isize && fy >= 0 && fy < self.height as isize;

            let second_within_bounds =
                sx >= 0 && sx < self.width as isize && sy >= 0 && sy < self.height as isize;

            let within_bounds = first_within_bounds || second_within_bounds;

            if first_within_bounds {
                self.antinodes.insert(Position {
                    x: fx as usize,
                    y: fy as usize,
                });
            }

            if second_within_bounds {
                self.antinodes.insert(Position {
                    x: sx as usize,
                    y: sy as usize,
                });
            }

            if !within_bounds || equal_dist {
                break;
            }

            iteration += 1;
        }

        if !equal_dist {
            self.antinodes.insert(first);
            self.antinodes.insert(second);
        }
    }

    fn add_antenna(&mut self, antenna: Antenna, equal_dist: bool) {
        let (position, frequency) = antenna;
        let mut resonations = vec![];

        for (existing_position, existing_frequency) in &self.antennas {
            if !(*existing_frequency).eq(&frequency) {
                continue;
            }

            resonations.push((*existing_position, position))
        }

        for (first, second) in resonations {
            self.add_antinodes(first, second, equal_dist);
        }

        self.antennas.push(antenna);
    }

    fn from_str(input: &str, equal_dist: bool) -> Self {
        let line_data: Vec<&str> = input.lines().collect();

        let mut map = Map {
            width: line_data.first().map(|line| line.len()).unwrap_or(0),
            height: line_data.len(),
            antennas: vec![],
            antinodes: HashSet::new(),
        };

        for (y, line) in line_data.into_iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let position = Position { x, y };

                if char.is_ascii_alphanumeric() {
                    map.add_antenna((position, char), equal_dist);
                }
            }
        }

        map
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position { x, y };

                if let Some((_, char)) = self.antennas.iter().find(|(pos, _char)| position.eq(pos))
                {
                    write!(f, "{}", char)?;
                    continue;
                }

                if self.antinodes.contains(&position) {
                    write!(f, "#")?;
                    continue;
                }

                write!(f, ".")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn part_1(input: &str) -> String {
    let map = Map::from_str(input, true);

    map.antinodes.len().to_string()
}
pub fn part_2(input: &str) -> String {
    let map = Map::from_str(input, false);

    map.antinodes.len().to_string()
}

#[cfg(test)]
mod test {
    use crate::{
        day08::{part_1, part_2},
        input, output,
    };

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(input::example::DAY_08), output::example::DAY_08_1);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(input::DAY_08), output::DAY_08_1)
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(input::example::DAY_08), output::example::DAY_08_2);
    }
    #[test]
    fn part_2_real() {
        assert_eq!(part_2(input::DAY_08), output::DAY_08_2)
    }
}
