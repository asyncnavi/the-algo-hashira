use std::io;

mod search;
mod operation;
mod set;
mod find;

fn main() {
    let arr: [isize;11] = [6,7,8,9,11,12,15,16,17,18,19];

    let result = find::multiple_missing(&arr);

    println!("{:?}", result);
}
