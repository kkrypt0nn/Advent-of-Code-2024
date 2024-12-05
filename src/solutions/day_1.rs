pub fn execute(test: bool) {
    println!("{}", part_one(test));
    println!("{}", part_two(test));
}

fn part_one(test: bool) -> isize {
    let content = aoc_rs::input::read_file(aoc_rs::input::get_path(1, test));
    let (left, right) = parse_input(content);

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part_two(test: bool) -> isize {
    let content = aoc_rs::input::read_file(aoc_rs::input::get_path(1, test));
    let (left, right) = parse_input(content);

    left.iter()
        .map(|l| l * right.iter().filter(|&r| r == l).count() as isize)
        .sum()
}

fn parse_input(s: Vec<String>) -> (Vec<isize>, Vec<isize>) {
    let mut left: Vec<isize> = Vec::new();
    let mut right: Vec<isize> = Vec::new();
    for line in s {
        let split = line.split("   ").collect::<Vec<&str>>();
        left.push(split[0].parse::<isize>().unwrap());
        right.push(split[1].parse::<isize>().unwrap());
    }
    left.sort();
    right.sort();
    (left, right)
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let result = part_one(true).to_string();
        assert_eq!(result, String::from("11"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(true).to_string();
        assert_eq!(result, String::from("31"));
    }
}
