use std::io;

mod search;
mod operation;
mod set;
mod find;

fn main() {
    let arr: [isize;13] = [6,7,8,9,11,11,12,15,16,17,18,19,19,];

    let result = find::duplicate(&arr);

    println!("{:?}", result);
}
