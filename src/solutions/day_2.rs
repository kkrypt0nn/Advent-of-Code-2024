pub fn execute(test: bool) {
    println!("{}", part_one(test));
    println!("{}", part_two(test));
}

fn part_one(test: bool) -> usize {
    let content = aoc_rs::input::read_file(aoc_rs::input::get_path(2, test));
    let lines = content
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|value| value.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    lines.iter().filter(|&line| is_safe(line, false)).count()
}

fn part_two(test: bool) -> usize {
    let content = aoc_rs::input::read_file(aoc_rs::input::get_path(2, test));
    let lines = content
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|value| value.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    lines.iter().filter(|&line| is_safe(line, true)).count()
}

fn is_safe(line: &[isize], tolerate_bad_level: bool) -> bool {
    let mut decrease_flag = false;
    for (i, num) in line.iter().enumerate() {
        if i == line.len() - 1 {
            break;
        }
        let current = *num;
        let next = line[i + 1];

        if i == 0 {
            decrease_flag = current > next;
        }

        let difference = (current - next).abs();
        if (decrease_flag && current < next)
            || (!decrease_flag && current > next)
            || !(1..=3).contains(&difference)
        {
            if !tolerate_bad_level {
                return false;
            }

            let start_at = if i == 0 { 0 } else { i - 1 };
            for j in start_at..line.len() {
                let mut new_line = line.to_owned();
                new_line.remove(j);
                if is_safe(&new_line, false) {
                    return true;
                }
            }

            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let result = part_one(true).to_string();
        assert_eq!(result, String::from("2"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(true).to_string();
        assert_eq!(result, String::from("4"));
    }
}
