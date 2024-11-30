pub mod day_1;
pub mod day_2;
pub mod day_9;
// pub mod day_10;
pub mod day_12;
pub mod day_13;
pub mod day_22;
pub mod utils;

fn main() {
    println!("Running day 1");
    day_1::solve();

    println!("Running day 2");
    day_2::solve();

    // TODO: All the way from day 2 to 9
    println!("Running day 9");
    day_9::solve();

    // println!("Running day 10");
    // day_10::solve();

    // TODO: Day 10 and 11 as well lmao
    println!("Running day 12");
    day_12::solve();

    println!("Running day 13");
    day_13::solve();

    println!("Running day 22");
    day_22::solve();
}
