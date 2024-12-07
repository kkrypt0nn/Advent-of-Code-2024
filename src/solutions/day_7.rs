use itertools::Itertools;

pub fn execute(test: bool) {
    let mut operators = Vec::from(['+', '*']);
    println!("{}", eval_input_for_operators(&operators, test));
    operators.push('|');
    println!("{}", eval_input_for_operators(&operators, test));
}

fn eval(numbers: &[usize], operations: &[&char]) -> usize {
    let mut result = numbers[0];
    for (i, op) in operations.iter().enumerate() {
        let number = numbers[i + 1];
        result = match op {
            '+' => result + number,
            '*' => result * number,
            '|' => format!("{}{}", result, number).parse::<usize>().unwrap(),
            _ => panic!("Should never happen"),
        }
    }
    result
}

fn eval_input_for_operators(operators: &Vec<char>, test: bool) -> usize {
    let content = aoc_rs::input::read_file(aoc_rs::input::get_path(7, test));

    content.iter().fold(0, |sum, line| {
        let mut split = line.split(": ");
        let target = split.next().unwrap().parse::<usize>().unwrap();
        let numbers: Vec<usize> = split
            .next()
            .unwrap()
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let mut operator_combinations = (0..numbers.len() - 1)
            .map(|_| operators)
            .multi_cartesian_product();

        if operator_combinations.any(|operators| eval(&numbers, &operators) == target) {
            sum + target
        } else {
            sum
        }
    })
}

#[cfg(test)]
mod test {
    use crate::solutions::day_7::eval_input_for_operators;

    #[test]
    fn test_part_one() {
        let operators = Vec::from(['+', '*']);
        let result = eval_input_for_operators(&operators, true).to_string();
        assert_eq!(result, String::from("3749"));
    }

    #[test]
    fn test_part_two() {
        let operators = Vec::from(['+', '*', '|']);
        let result = eval_input_for_operators(&operators, true).to_string();
        assert_eq!(result, String::from("11387"));
    }
}
