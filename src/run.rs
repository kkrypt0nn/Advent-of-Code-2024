use crate::solutions;

pub fn run_day(day: usize, test: bool) {
    match day {
        1 => solutions::day_1::execute(test),
        2 => solutions::day_2::execute(test),
        3 => solutions::day_3::execute(test),
        4 => solutions::day_4::execute(test),
        5 => solutions::day_5::execute(test),
        6 => solutions::day_6::execute(test),
        // New day here
        _ => println!("Day {} is not existing", day),
    }
}
