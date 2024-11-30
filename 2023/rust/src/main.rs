pub mod day_1;
pub mod day_2;
pub mod day_9;
pub mod utils;

fn main() {

    println!("Running day 1");
    day_1::solve();

    println!("Running day 2");
    day_2::solve();

    // TODO: All the way from day 2 to 9
    println!("Running day 9");
    day_9::solve();
}
