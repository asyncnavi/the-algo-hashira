
pub fn factorial(n : isize) ->  isize {
    return if n == 1 {
        1
    } else {
        factorial(n - 1) * n    
    }

}

fn main() {
    let fact = factorial(5);
    println!("{fact}");
}
