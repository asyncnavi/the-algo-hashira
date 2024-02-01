use std::io;

mod search;
mod operation;


fn main() {
    let values: [isize;7] = [12,16,18,22,46,78,80];
    let is_sorted =  operation::operation::is_sorted(&values);
    println!("{is_sorted}");
}
