use std::io;

mod search;
mod operation;


fn main() {
    let mut values: [isize;7] = [-9,6,4,-1,2,-3,1];
    operation::operation::shift_signs(&mut values);
    println!("{:?}", values);
}
