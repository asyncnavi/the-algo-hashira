mod taylor_series;
mod fibonacci_series;

use taylor_series::taylor_series::{sum ,sum_in_linear_time};
fn main() {

    println!("################# TAYLOR SERIES");
    let sum_of_series = unsafe { sum(2, 10) };
    let sum_of_series_l = unsafe { sum_in_linear_time(2, 10) };
    let fibo = fibonacci_series::fibonacci_series::sum(6);
    println!("{sum_of_series}, {sum_of_series_l}, {fibo}");

}
