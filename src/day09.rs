use std::{collections::HashSet, fmt::Display};

type DiskBlock = Option<usize>;

struct DiskMap {
    blocks: Vec<DiskBlock>,
}

impl DiskMap {
    fn new(layout: &str) -> DiskMap {
        let mut map = DiskMap { blocks: vec![] };

        let bytes = layout.trim().as_bytes();

        for (block_id, chunk) in bytes.chunks(2).enumerate() {
            let count: u8 = chunk[0] - b'0';
            let free_space: u8 = if chunk.len() == 2 { chunk[1] - b'0' } else { 0 };

            (0..count).for_each(|_| map.blocks.push(Some(block_id)));
            (0..free_space).for_each(|_| map.blocks.push(None));
        }

        map
    }

    fn compact(&mut self) {
        let mut data_end = self.blocks.len();

        for free_space_index in 0..self.blocks.len() {
            if free_space_index >= data_end {
                self.blocks.truncate(data_end);
                break;
            }

            if self.blocks[free_space_index].is_some() {
                continue;
            }

            for index in ((free_space_index + 1)..data_end).rev() {
                if self.blocks[index].is_none() {
                    continue;
                }
                data_end = index;
                self.blocks.swap(index, free_space_index);
                break;
            }
        }
    }

    fn find_free_space(&self, size: usize, end: usize) -> Option<usize> {
        if size >= end || end > self.blocks.len() {
            return None;
        }

        let mut index = 0;

        'scan: while index + size - 1 < end {
            for check in 0..size {
                if self.blocks[index + check].is_some() {
                    index += check + 1;
                    continue 'scan;
                }
            }

            assert!(self.blocks[index + size - 1].is_none());
            assert!(self.blocks[index].is_none());
            if index > 0 {
                assert!(self.blocks[index - 1].is_some());
            }
            return Some(index);
        }

        None
    }

    fn move_file(&mut self, from: usize, to: usize, size: usize) {
        for index in 0..size {
            self.blocks.swap(from + index, to + index);
        }
    }

    fn compact_2(&mut self) {
        let mut file_end = self.blocks.len() - 1;
        let mut max_file_id = usize::MAX;

        while file_end > 0 {
            let Some(file_id) = self.blocks[file_end] else {
                file_end -= 1;
                continue;
            };

            if file_id >= max_file_id {
                file_end -= 1;
                continue;
            }

            max_file_id = file_id;

            let mut file_start = file_end;

            for index in (0..file_end).rev() {
                let Some(start_id) = self.blocks[index] else {
                    break;
                };
                if start_id != file_id {
                    break;
                }
                file_start = index;
            }

            let size = file_end - file_start + 1;

            // At this point, index and size of file to try to move is known

            if let Some(free_space) = self.find_free_space(size, file_start) {
                self.move_file(file_start, free_space, size);
            }
            if file_start <= 1 {
                break;
            }

            file_end = file_start - 1;
        }

        // // OPTIMIZATION: Use a cursor and jumps instead of continues
        // for file_end in (1..self.blocks.len()).rev() {
        //     let Some(file_id) = self.blocks[file_end] else {
        //         continue;
        //     };

        //     if processed.contains(&file_id) {
        //         continue;
        //     }

        //     let mut file_start = file_end;

        //     for index in (0..file_end).rev() {
        //         let Some(start_id) = self.blocks[index] else {
        //             break;
        //         };
        //         if start_id != file_id {
        //             break;
        //         }
        //         file_start = index;
        //     }

        //     let size = file_end - file_start + 1;

        //     // At this point, index and size of file to try to move is known

        //     // TODO: be more precise
        //     if size >= file_start {
        //         // We cannot move this one there is no space to the left for it
        //         continue;
        //     }

        //     if let Some(free_space) = self.find_free_space(size, file_start) {
        //         self.move_file(file_start, free_space, size);
        //     }

        //     processed.insert(file_id);
        // }
    }

    fn checksum(&self) -> usize {
        let mut checksum = 0;
        for (index, block) in self.blocks.iter().enumerate() {
            match block {
                Some(id) => checksum += index * id,
                None => continue,
            }
        }

        checksum
    }
}

fn block_id_char(value: usize) -> char {
    match value {
        0..=9 => (b'0' + value as u8) as char,
        10..=35 => (b'a' + (value as u8 - 10)) as char,
        36..61 => (b'A' + (value as u8 - 36)) as char,
        _ => '?',
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            match block {
                Some(id) => write!(f, "{}", block_id_char(*id))?,
                None => write!(f, ".")?,
            }
        }

        Ok(())
    }
}

pub fn part_1(input: &str) -> String {
    let mut map = DiskMap::new(input);

    map.compact();
    map.checksum().to_string()
}

pub fn part_2(input: &str) -> String {
    let mut map = DiskMap::new(input);

    map.compact_2();
    map.checksum().to_string()
}

#[cfg(test)]
mod test {
    use crate::{
        day09::{part_1, part_2},
        input, output,
    };

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(input::example::DAY_09), output::example::DAY_09_1);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(input::DAY_09), output::DAY_09_1)
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(input::example::DAY_09), output::example::DAY_09_2);
    }
    #[test]
    fn part_2_real() {
        assert_eq!(part_2(input::DAY_09), output::DAY_09_2)
    }
}
