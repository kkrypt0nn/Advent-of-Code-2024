use std::collections::{HashMap, HashSet};

use aoc_rs::point::Point2;
use itertools::Itertools;

struct City {
    freq_antennas: HashMap<char, Vec<Point2>>,
    dimensions: (isize, isize),
}

impl City {
    fn new(test: bool) -> Self {
        let content = aoc_rs::input::read_file(aoc_rs::input::get_path(8, test));
        let mut freq_antennas: HashMap<char, Vec<Point2>> = HashMap::new();
        for (y, line) in content.iter().enumerate() {
            for (x, frequency) in line.chars().enumerate() {
                if frequency != '.' {
                    freq_antennas
                        .entry(frequency)
                        .or_insert_with(Vec::new)
                        .push(Point2::new(x as isize, y as isize));
                }
            }
        }

        City {
            freq_antennas,
            dimensions: (content[0].len() as isize, content.len() as isize),
        }
    }

    fn is_in_bounds(&self, point: Point2) -> bool {
        point.x >= 0 && point.x < self.dimensions.0 && point.y >= 0 && point.y < self.dimensions.1
    }

    fn get_antinodes(&self) -> usize {
        let mut antinodes: HashSet<Point2> = HashSet::new();
        for antennas in self.freq_antennas.values() {
            for [antenna1, antenna2] in antennas
                .iter()
                .combinations(2)
                .map(|combination| [*combination[0], *combination[1]])
            {
                let diff = antenna2 - antenna1;
                let potential_antinodes = [antenna2 + diff, antenna1 - diff];
                for antinode in potential_antinodes {
                    if self.is_in_bounds(antinode) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
        antinodes.len()
    }

    fn get_antinodes_in_line(&self) -> usize {
        let mut antinodes: HashSet<Point2> = HashSet::new();
        for antennas in self.freq_antennas.values() {
            for [antenna1, antenna2] in antennas
                .iter()
                .combinations(2)
                .map(|combination| [*combination[0], *combination[1]])
            {
                let diff = antenna2 - antenna1;
                for step in 0.. {
                    let step_diff = Point2::new(diff.x * step, diff.y * step);
                    let potential_antinodes = [antenna2 + step_diff, antenna1 - step_diff];
                    let mut antinode_in_bounds = false;
                    for antinode in potential_antinodes {
                        if self.is_in_bounds(antinode) {
                            antinodes.insert(antinode);
                            antinode_in_bounds = true;
                        }
                    }
                    if !antinode_in_bounds {
                        break;
                    }
                }
            }
        }
        antinodes.len()
    }
}

pub fn execute(test: bool) {
    let city = City::new(test);
    println!("{}", city.get_antinodes());
    println!("{}", city.get_antinodes_in_line());
}

#[cfg(test)]
mod test {
    use crate::solutions::day_8::City;

    #[test]
    fn test_part_one() {
        let city = City::new(true);
        let result = city.get_antinodes().to_string();
        assert_eq!(result, String::from("14"));
    }

    #[test]
    fn test_part_two() {
        let city = City::new(true);
        let result = city.get_antinodes_in_line().to_string();
        assert_eq!(result, String::from("34"));
    }
}
