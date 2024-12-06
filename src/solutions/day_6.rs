use aoc_rs::{direction::Direction, orientation::Orientation, parsing};

struct Lab {
    map: Vec<Vec<char>>,
    guard_pos: (usize, usize),
    starting_pos: (usize, usize),
    facing: Direction,
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
        }
    }

    fn mark_as_visited(&mut self, pos: (usize, usize)) {
        self.map[pos.1][pos.0] = 'X';
    }

    fn is_obstacle_in_front(&self) -> bool {
        let direction = self.facing.to_vector2();
        let (x, y) = self.guard_pos;
        let new_x = x as isize + direction.x;
        let new_y = y as isize + direction.y;
        self.map[new_y as usize][new_x as usize] == '#'
    }

    fn step_forward(&mut self) {
        let direction_vec = self.facing.to_vector2();
        let (x, y) = self.guard_pos;
        self.guard_pos = (
            (x as isize + direction_vec.x) as usize,
            (y as isize + direction_vec.y) as usize,
        );
    }

    fn patrol_guard(&mut self) {
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

            if self.is_obstacle_in_front() {
                self.facing = self.facing.rotate_90(Orientation::Right);
            } else {
                self.step_forward();
            }
        }
    }

    fn simulate_with_obstruction(&mut self, obstruction: (usize, usize)) -> bool {
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

            if self.is_obstacle_in_front() {
                self.facing = self.facing.rotate_90(Orientation::Right);
            } else {
                self.step_forward();
            }
        }

        self.map[obstruction.1][obstruction.0] = '.';
        false
    }

    fn simualte_all_obstructions(&mut self) -> usize {
        let mut valid_positions = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if self.map[y][x] == '.' {
                    if self.simulate_with_obstruction((x, y)) {
                        valid_positions += 1;
                    }
                }
            }
        }
        valid_positions
    }
}

pub fn execute(test: bool) {
    let mut lab = Lab::new(test);
    lab.patrol_guard();
    println!(
        "{}",
        lab.map
            .iter()
            .map(|l| l.iter().filter(|&&c| c == 'X').count())
            .sum::<usize>()
    );

    let mut lab = Lab::new(test);
    println!("{}", lab.simualte_all_obstructions());
}

#[cfg(test)]
mod test {
    use crate::solutions::day_6::Lab;

    #[test]
    fn test_part_one() {
        let mut lab = Lab::new(true);
        lab.patrol_guard();
        let result = lab
            .map
            .iter()
            .map(|l| l.iter().filter(|&&c| c == 'X').count())
            .sum::<usize>()
            .to_string();
        assert_eq!(result, String::from("41"));
    }

    #[test]
    fn test_part_two() {
        let mut lab = Lab::new(true);
        let result = lab.simualte_all_obstructions().to_string();
        assert_eq!(result, String::from("6"));
    }
}
