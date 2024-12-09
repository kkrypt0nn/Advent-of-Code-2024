pub fn execute(test: bool) {
    println!("{}", part_one(test));
    println!("{}", part_two(test));
}

fn parse_disk(test: bool) -> (Vec<String>, usize) {
    let content = aoc_rs::input::read_file_string(aoc_rs::input::get_path(9, test));
    let mut disk: Vec<String> = Vec::new();
    let mut id = 0;
    for (i, n) in content
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
    {
        if i % 2 == 0 {
            disk.extend(vec![id.to_string(); n]);
            id += 1;
        } else {
            disk.extend(vec![".".to_string(); n]);
        }
    }
    (disk, id)
}

fn part_one(test: bool) -> usize {
    let mut disk = parse_disk(test).0;
    for i in (0..disk.len()).rev() {
        if let Some(pos) = disk.iter().position(|d| d == ".") {
            if pos >= i {
                break;
            }
            disk.swap(pos, i);
        }
    }

    disk.iter()
        .enumerate()
        .filter_map(|(i, n)| n.parse::<usize>().ok().map(|parsed| i * parsed))
        .sum()
}

fn part_two(test: bool) -> usize {
    let mut disk = parse_disk(test).0;
    let id = parse_disk(test).1;

    for file_id in (0..id).rev() {
        let mut file_block: Vec<usize> = Vec::new();
        let mut free_positions = Vec::new();

        let mut positions = Vec::new();
        for (i, s) in disk.iter().enumerate() {
            // Find the block for the current file id
            if s == &file_id.to_string() {
                file_block.push(i);
            }

            // Find the free positions (consecutive ".")
            if s == "." {
                positions.push(i);
            } else if !positions.is_empty() {
                free_positions.push(positions.clone());
                positions.clear();
            }
        }

        free_positions.retain(|free_position| {
            // Just want to keep the free postions
            // * That are to the left of the file block
            // * Where the entire file block can fit in
            free_position[0] < file_block[0] && free_position.len() >= file_block.len()
        });
        free_positions.iter().next().map(|free_position| {
            for (file_i, free_i) in file_block.iter().zip(free_position) {
                disk.swap(*file_i, *free_i);
            }
        });
    }

    disk.iter()
        .enumerate()
        .filter_map(|(i, n)| n.parse::<usize>().ok().map(|parsed| i * parsed))
        .sum()
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let result = part_one(true).to_string();
        assert_eq!(result, String::from("1928"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(true).to_string();
        assert_eq!(result, String::from("2858"));
    }
}
