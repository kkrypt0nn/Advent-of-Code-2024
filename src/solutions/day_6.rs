use std::collections::HashSet;

use aoc_rs::{direction::Direction, orientation::Orientation, parsing};

struct Lab {
    map: Vec<Vec<char>>,
    guard_pos: (usize, usize),
    facing: Direction,
    starting_pos: (usize, usize),
    visited_pos: HashSet<(usize, usize)>,
}

impl Lab {
    fn new(test: bool) -> Self {
        let content = aoc_rs::input::read_file(aoc_rs::input::get_path(6, test));
        let map = content
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let guard_pos = parsing::find_character(content, '^');

        Lab {
            map,
            starting_pos: guard_pos,
            guard_pos,
            facing: Direction::North,
            visited_pos: HashSet::new(),
        }
    }

    fn mark_as_visited(&mut self, pos: (usize, usize)) {
        self.visited_pos.insert((pos.0, pos.1));
    }

    fn patrol_guard(&mut self) -> usize {
        loop {
            let direction = self.facing.to_vector2();
            let (x, y) = self.guard_pos;
            self.mark_as_visited((x, y));
            let new_pos = (x as isize + direction.x, y as isize + direction.y);

            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 as usize >= self.map[0].len()
                || new_pos.1 as usize >= self.map.len()
            {
                break;
            }

            if self.map[new_pos.1 as usize][new_pos.0 as usize] == '#' {
                self.facing = self.facing.rotate_90(Orientation::Right);
            } else {
                self.guard_pos = (new_pos.0 as usize, new_pos.1 as usize);
            }
        }
        self.visited_pos.len()
    }

    fn get_loop_with_obstruction(&mut self, obstruction: (usize, usize)) -> bool {
        self.map[obstruction.1][obstruction.0] = '#';
        let mut visited_pos_facing = Vec::new();

        // Reset guard pos & facing
        self.guard_pos = self.starting_pos;
        self.facing = Direction::North;

        loop {
            // If the guard visits the same position with the same facing, it is in a loop
            let pos_facing = (self.guard_pos, self.facing);
            if visited_pos_facing.contains(&pos_facing) {
                self.map[obstruction.1][obstruction.0] = '.';
                return true;
            }
            visited_pos_facing.push(pos_facing);

            let direction = self.facing.to_vector2();
            let (x, y) = self.guard_pos;
            let new_pos = (x as isize + direction.x, y as isize + direction.y);

            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 as usize >= self.map[0].len()
                || new_pos.1 as usize >= self.map.len()
            {
                break;
            }

            if self.map[new_pos.1 as usize][new_pos.0 as usize] == '#' {
                self.facing = self.facing.rotate_90(Orientation::Right);
            } else {
                self.guard_pos = (new_pos.0 as usize, new_pos.1 as usize);
            }
        }

        self.map[obstruction.1][obstruction.0] = '.';
        false
    }

    fn simulate_with_new_obstructions(&mut self) -> usize {
        self.visited_pos
            .clone()
            .iter()
            .filter(|(x, y)| self.get_loop_with_obstruction((*x, *y)))
            .count()
    }
}

pub fn execute(test: bool) {
    let mut lab = Lab::new(test);
    let result = lab.patrol_guard();
    println!("{}", result);
    println!("{}", lab.simulate_with_new_obstructions());
}

#[cfg(test)]
mod test {
    use crate::solutions::day_6::Lab;

    #[test]
    fn test_part_one() {
        let mut lab = Lab::new(true);
        let result = lab.patrol_guard().to_string();
        assert_eq!(result, String::from("41"));
    }

    #[test]
    fn test_part_two() {
        let mut lab = Lab::new(true);
        lab.patrol_guard();
        let result = lab.simulate_with_new_obstructions().to_string();
        assert_eq!(result, String::from("6"));
    }
}
