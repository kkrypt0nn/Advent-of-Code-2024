use std::{
    fs::{self, File},
    io::Write,
};

const DEFAULT_MAIN_RS_FILE: &'static str = r###"pub fn execute(test: bool) {
    println!("{}", part_one(test));
    println!("{}", part_two(test));
}

fn part_one(test: bool) -> usize {
    let content = aoc_rs::input::read_file(aoc_rs::input::get_path({{day}}, test));
    content.len()
}

fn part_two(test: bool) -> usize {
    let content = aoc_rs::input::read_file(aoc_rs::input::get_path({{day}}, test));
    content.len()
}"###;

pub fn new_day(day: usize) {
    // Create input file
    println!("Creating input file for day {}", day);
    if let Err(e) = File::create(format!("input/{}.txt", day)) {
        panic!("{}", e);
    }

    // Create test input file
    println!("Creating test input file for day {}", day);
    if let Err(e) = File::create(format!("input/{}_test.txt", day)) {
        panic!("{}", e);
    }

    // Create new file for the day
    println!("Creating folder for day {}", day);
    let mut file = File::create(format!("src/solutions/day_{}.rs", day)).unwrap();
    file.write_all(
        DEFAULT_MAIN_RS_FILE
            .replace("{{day}}", day.to_string().as_str())
            .as_bytes(),
    )
    .unwrap();

    // Update run.rs file
    println!("Adding day {} to run options", day);
    let content = fs::read_to_string("src/run.rs").unwrap().replace(
        "// New day here",
        format!(
            "{0} => solutions::day_{0}::execute(test),\n\t\t// New day here",
            day
        )
        .as_str(),
    );
    let mut file = File::create("src/run.rs").unwrap();
    file.write_all(content.as_bytes()).unwrap();

    // Update solutions/mod.rs file
    println!("Adding day {} to solution mod file", day);
    let content = format!(
        "{}\npub mod day_{};",
        fs::read_to_string("src/solutions/mod.rs").unwrap(),
        day
    );
    let mut file = File::create("src/solutions/mod.rs").unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
