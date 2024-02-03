use std::io;

mod search;
mod operation;


fn main() {
    let arr: [isize;6] = [1,2,3,4,5,6];
    let s_arr : [isize;6] = [3,4,5,6,7,8];

    let result = operation::operation::diff(&arr,&s_arr);

    println!("{:?}", result);
}
