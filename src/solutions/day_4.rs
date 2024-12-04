pub fn execute(test: bool) {
    println!("{}", part_one(test));
    println!("{}", part_two(test));
}

fn part_one(test: bool) -> usize {
    let content = aoc_rs::input::read_file(aoc_rs::input::get_path(4, test));
    // x,y
    let directions_start = [
        (1, -1),  // Right up
        (1, 0),   // Right
        (1, 1),   // Right down
        (0, 1),   // Down
        (-1, 1),  // Left down
        (-1, 0),  // Left
        (-1, -1), // Left up
        (0, -1),  // Up
    ];
    let mut sum = 0;
    for (y, line) in content.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == 'X' {
                for start in directions_start {
                    if check_direction(content.clone(), (x, y), start) {
                        sum += 1;
                    }
                }
            }
        }
    }
    sum
}

fn check_direction(
    content: Vec<String>,
    x_char_pos: (usize, usize),
    direction_start: (isize, isize),
) -> bool {
    let mut next_chars = String::new();
    for step in 1..=3 {
        let new_x = x_char_pos.0 as isize + direction_start.0 * step;
        let new_y = x_char_pos.1 as isize + direction_start.1 * step;

        if new_x < 0
            || new_y < 0
            || new_y >= content.len() as isize
            || content[new_y as usize]
                .chars()
                .nth(new_x as usize)
                .is_none()
        {
            return false;
        }

        next_chars.push(content[new_y as usize].chars().nth(new_x as usize).unwrap());
    }
    next_chars == "MAS"
}

fn part_two(test: bool) -> usize {
    let content = aoc_rs::input::read_file(aoc_rs::input::get_path(4, test));
    let mut sum = 0;
    for (y, line) in content.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == 'A' {
                if y == 0 || x == 0 || y >= content.len() - 1 || x >= line.len() - 1 {
                    continue;
                }
                let surrounding_chars: String = [
                    content[y - 1].chars().nth(x + 1).unwrap(), // Up right
                    content[y + 1].chars().nth(x + 1).unwrap(), // Down right
                    content[y + 1].chars().nth(x - 1).unwrap(), // Down left
                    content[y - 1].chars().nth(x - 1).unwrap(), // Up left
                ]
                .into_iter()
                .collect();
                if matches!(
                    surrounding_chars.as_str(),
                    "SSMM" | "MSSM" | "MMSS" | "SMMS"
                ) {
                    sum += 1;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let result = part_one(true).to_string();
        assert_eq!(result, String::from("18"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(true).to_string();
        assert_eq!(result, String::from("9"));
    }
}
