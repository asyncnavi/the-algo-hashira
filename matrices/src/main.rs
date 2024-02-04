mod diagonal;

fn main() {
    let mut diag = diagonal::Diagonal::new(5);
    diag.set(2, 2, 42);
    diag.set(3, 3, 99);
    println!("Diagonal element at (2, 2): {:?}", diag.get(2, 2));
    println!("Diagonal element at (3, 3): {:?}", diag.get(3, 3));
    println!("Diagonal element at (1, 1): {:?}", diag.get(1, 1));
}
