use std::io;

mod search;
mod operation;


fn main() {
    let mut values: [isize;7] = [12,16,18,22,46,78,50];
    println!("Array before rev ... : {:?}", values);
    operation::operation::rev(&mut values);
    println!("Array after rev ... : {:?}", values);

}
