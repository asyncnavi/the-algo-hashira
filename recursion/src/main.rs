mod taylor_series;
mod fibonacci_series;
mod toh;
mod ncr;

use toh::toh::tower_of_hanoi;
fn main() {

    println!("################# TAYLOR SERIES");
    tower_of_hanoi(3,'A','B', 'C');
}
