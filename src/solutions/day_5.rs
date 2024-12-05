struct PrintQueue {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

impl PrintQueue {
    fn new(test: bool) -> Self {
        let content = aoc_rs::input::read_file_string(aoc_rs::input::get_path(5, test));
        let mut sections = content.split("\n\n");
        let rules = if let Some(rule_lines) = sections.next() {
            rule_lines
                .lines()
                .map(|line| {
                    let parts: Vec<&str> = line.split('|').collect();
                    (
                        parts[0].parse::<usize>().unwrap(),
                        parts[1].parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<(usize, usize)>>()
        } else {
            Vec::new()
        };
        let updates = if let Some(update_lines) = sections.next() {
            update_lines
                .lines()
                .map(|line| {
                    line.split(',')
                        .map(|value| value.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        } else {
            Vec::new()
        };

        PrintQueue { rules, updates }
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
        self.updates
            .iter()
            .filter(|update| self.is_correct_order(update))
            .map(|update| update[update.len() / 2])
            .sum()
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
        self.updates
            .iter()
            .filter(|update| !self.is_correct_order(update))
            .map(|update| {
                let sorted_update = self.sort_update(update.clone());
                sorted_update[sorted_update.len() / 2]
            })
            .sum()
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
