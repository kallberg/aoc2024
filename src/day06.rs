use core::panic;
use rayon::prelude::*;
use std::{collections::HashSet, fmt::Display};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn up(&self) -> Self {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Self {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Self {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Self {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Clone)]
struct Guard {
    width: usize,
    height: usize,
    position: Position,
    obstructions: HashSet<Position>,
    direction: Direction,
    visited: HashSet<Position>,
    history: Vec<(Position, Direction)>,
    turns: HashSet<Position>,
    in_loop: bool,
    extra_obstruction: Option<Position>,
}

impl From<&str> for Guard {
    fn from(input: &str) -> Self {
        let mut width = 0;
        let mut obstructions = HashSet::new();
        let mut position = Position { x: 0, y: 0 };
        let mut visited = HashSet::new();
        let mut history = vec![];
        let turns = HashSet::new();
        let mut height = 0;
        let direction = Direction::Up;
        let in_loop = false;
        let extra_obstruction = None;

        for (y, line) in input.lines().enumerate() {
            if width == 0 {
                width = line.len();
            } else {
                assert!(width == line.len());
            }

            for (x, character) in line.chars().enumerate() {
                match character {
                    '.' => {}
                    '#' => {
                        obstructions.insert(Position { x, y });
                    }
                    '^' => {
                        position = Position { x, y };
                    }
                    _ => {
                        panic!("encountered not map character {} in map", character);
                    }
                }
            }
            height += 1;
        }

        visited.insert(position);
        history.push((position, direction));

        Guard {
            width,
            height,
            obstructions,
            position,
            direction: Direction::Up,
            visited,
            history,
            turns,
            in_loop,
            extra_obstruction,
        }
    }
}

impl Guard {
    fn up(&self) -> Option<Position> {
        if self.position.y == 0 {
            return None;
        }

        Some(self.position.up())
    }

    fn down(&self) -> Option<Position> {
        if self.position.y + 1 >= self.height {
            return None;
        }

        Some(self.position.down())
    }

    fn left(&self) -> Option<Position> {
        if self.position.x == 0 {
            return None;
        }

        Some(self.position.left())
    }

    fn right(&self) -> Option<Position> {
        if self.position.x + 1 >= self.width {
            return None;
        }

        Some(self.position.right())
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    fn patrol(&mut self) {
        while self.step() {}
    }

    fn forward(&self) -> Option<Position> {
        match self.direction {
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::Up => self.up(),
            Direction::Down => self.down(),
        }
    }

    fn step(&mut self) -> bool {
        let Some(forward) = self.forward() else {
            return false;
        };

        if self.history.contains(&(forward, self.direction)) {
            self.in_loop = true;
            return false;
        };

        if self.obstructions.contains(&forward) {
            self.turns.insert(self.position);
            self.turn();
            return self.step();
        }

        self.position = forward;

        self.visited.insert(self.position);
        self.history.push((self.position, self.direction));

        true
    }
}

impl Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position { x, y };

                if self.extra_obstruction.eq(&Some(position)) {
                    write!(f, "O")?;
                    continue;
                }

                if self.obstructions.contains(&position) {
                    write!(f, "#")?;
                    continue;
                }

                if self.turns.contains(&position) {
                    write!(f, "+")?;
                    continue;
                }

                if self.visited.contains(&position) {
                    let mut left = false;
                    let mut right = false;
                    let mut up = false;
                    let mut down = false;

                    self.history.clone().into_iter().for_each(|(p, direction)| {
                        if p.eq(&position) {
                            match direction {
                                Direction::Left => left = true,
                                Direction::Right => right = true,
                                Direction::Up => up = true,
                                Direction::Down => down = true,
                            }
                        }
                    });

                    write!(
                        f,
                        "{}",
                        match (left, right, up, down) {
                            (true, false, false, false) => '-',
                            (false, true, false, false) => '-',
                            (false, false, true, false) => '|',
                            (false, false, false, true) => '|',
                            _ => '+',
                        }
                    )?;
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
    let mut guard = Guard::from(input);

    guard.patrol();

    guard.visited.len().to_string()
}

pub fn part_2(input: &str) -> String {
    let mut patrol_guard = Guard::from(input);
    let guard_at_start = patrol_guard.clone();
    let mut obstructions = HashSet::new();
    let mut guards = vec![];

    while patrol_guard.step() {
        if !obstructions.insert(patrol_guard.position) {
            continue;
        }

        let mut possible_guard = guard_at_start.clone();
        possible_guard.obstructions.insert(patrol_guard.position);
        possible_guard.extra_obstruction = Some(patrol_guard.position);

        guards.push(possible_guard);
    }

    let loops: usize = guards
        .par_iter_mut()
        .map(|guard| {
            guard.patrol();
            if guard.in_loop {
                return 1;
            }

            0
        })
        .sum();

    loops.to_string()
}

#[cfg(test)]
mod test {
    use crate::{
        day06::{part_1, part_2},
        input, output,
    };

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(input::example::DAY_06), output::example::DAY_06_1);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(input::DAY_06), output::DAY_06_1)
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(input::example::DAY_06), output::example::DAY_06_2);
    }
    #[test]
    fn part_2_real() {
        assert_eq!(part_2(input::DAY_06), output::DAY_06_2)
    }
}
