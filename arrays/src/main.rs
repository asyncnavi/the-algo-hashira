use std::io;

mod search;


fn main() {
    let values: [isize;7] = [12,16,18,22,46,78,50];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read input");
    let key = input_line.trim().parse().expect("Input is not an integer");

    match search::search::binary_search_rec(&values, key) {
        Some(index) => println!("{key} is found at index {index}"),
        None => println!("{key} is not found.")
    }

}
