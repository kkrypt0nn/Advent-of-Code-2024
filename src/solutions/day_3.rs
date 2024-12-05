use regex::Regex;

pub fn execute(test: bool) {
    println!("{}", part_one(test));
    println!("{}", part_two(test));
}

fn part_one(test: bool) -> usize {
    let content = aoc_rs::input::read_file_string(aoc_rs::input::get_path(3, test));
    let re = Regex::new(r"mul\((?P<a>\d{1,3}),(?P<b>\d{1,3})\)").unwrap();
    re.captures_iter(&content)
        .map(|caps| {
            let a = caps["a"].parse::<usize>().unwrap();
            let b = caps["b"].parse::<usize>().unwrap();
            a * b
        })
        .sum()
}

fn part_two(test: bool) -> usize {
    let content = aoc_rs::input::read_file_string(aoc_rs::input::get_path(3, test));
    let re = Regex::new(r"mul\((?P<a>\d{1,3}),(?P<b>\d{1,3})\)|do(n't)?\(\)").unwrap();

    let mut enabled = 1;
    re.captures_iter(&content)
        .map(|c| {
            let mut result = 0;
            match c.get(0).unwrap().as_str() {
                "don't()" => enabled = 0,
                "do()" => enabled = 1,
                _ => {
                    let a = c.name("a").unwrap().as_str().parse::<usize>().unwrap();
                    let b = c.name("b").unwrap().as_str().parse::<usize>().unwrap();
                    result = a * b * enabled;
                }
            };
            result
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let result = part_one(true).to_string();
        assert_eq!(result, String::from("161"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(true).to_string();
        assert_eq!(result, String::from("48"));
    }
}
