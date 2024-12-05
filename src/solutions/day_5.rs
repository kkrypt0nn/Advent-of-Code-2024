struct PrintQueue {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

impl PrintQueue {
    fn new(test: bool) -> Self {
        let mut print_queue = PrintQueue {
            rules: Vec::new(),
            updates: Vec::new(),
        };
        print_queue.parse_input(test);
        print_queue
    }

    fn parse_input(&mut self, test: bool) {
        let content = aoc_rs::input::read_file_string(aoc_rs::input::get_path(5, test));
        let mut sections = content.split("\n\n");

        self.rules = if let Some(rule_lines) = sections.next() {
            rule_lines
                .lines()
                .map(|line| {
                    let parts: Vec<&str> = line.split('|').collect();
                    (
                        parts[0].parse::<usize>().unwrap(),
                        parts[1].parse::<usize>().unwrap(),
                    )
                })
                .collect()
        } else {
            Vec::new()
        };
        self.updates = if let Some(update_lines) = sections.next() {
            update_lines
                .lines()
                .map(|line| {
                    line.split(',')
                        .map(|value| value.parse::<usize>().unwrap())
                        .collect()
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    fn is_correct_order(&self, update: &[usize]) -> bool {
        self.rules.iter().all(|&(rule_a, rule_b)| {
            match (
                update.iter().position(|&u| u == rule_a),
                update.iter().position(|&u| u == rule_b),
            ) {
                (Some(pos_a), Some(pos_b)) => pos_a < pos_b,
                _ => true,
            }
        })
    }

    fn part_one(&self) -> usize {
        let mut sum = 0;
        for update in &self.updates {
            if self.is_correct_order(update) {
                if let Some(&middle) = update.get(update.len() / 2) {
                    sum += middle;
                }
            }
        }

        sum
    }

    fn sort_update(&self, mut update: Vec<usize>) -> Vec<usize> {
        while !self.is_correct_order(&update) {
            for &(rule_a, rule_b) in &self.rules {
                let pos_a = update.iter().position(|&u| u == rule_a);
                let pos_b = update.iter().position(|&u| u == rule_b);
                if let (Some(a), Some(b)) = (pos_a, pos_b) {
                    if a > b {
                        update.swap(a, b);
                    }
                }
            }
        }
        update
    }

    fn part_two(&self) -> usize {
        let mut sum = 0;
        for update in &self.updates {
            if !self.is_correct_order(update) {
                let sorted_update = self.sort_update(update.clone());
                if let Some(&middle) = sorted_update.get(sorted_update.len() / 2) {
                    sum += middle;
                }
            }
        }

        sum
    }
}

pub fn execute(test: bool) {
    let print_queue = PrintQueue::new(test);
    println!("{}", print_queue.part_one());
    println!("{}", print_queue.part_two());
}

#[cfg(test)]
mod test {
    use crate::solutions::day_5::PrintQueue;

    #[test]
    fn test_part_one() {
        let print_queue = PrintQueue::new(true);
        let result = print_queue.part_one().to_string();
        assert_eq!(result, String::from("143"));
    }

    #[test]
    fn test_part_two() {
        let print_queue = PrintQueue::new(true);
        let result = print_queue.part_two().to_string();
        assert_eq!(result, String::from("123"));
    }
}
