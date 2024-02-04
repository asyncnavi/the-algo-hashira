use std::collections::HashMap;
use crate::operation;

pub fn single_missing(a: &[isize]) -> Option<isize> {
    let diff = a[0] - 0;

    for i in 0..a.len() {
        if a[i] - i as isize != diff {
            return Some(i as isize + diff);
        }
    }

    None
}

pub fn multiple_missing(a : &[isize]) -> Vec<isize>{
    let mut res = Vec::new();
    let mut diff = a[0] - 0;

    for i in 0..a.len() {
        if a[i] - i as isize != diff {
            while (a[i] - i as isize)  > diff {
                res.push(i as isize +diff);
                diff+=1;
            }
        }
    }

    res
}


pub fn multiple_missing_using_hash(a: &[isize]) -> Vec<isize> {
    let mut indexes: HashMap<isize, bool> = HashMap::new();
    let mut res = Vec::new();

    for &element in a {
        indexes.insert(element, true);
    }

    let max = operation::max(a);
    let min = a[0];
    for i in min..max {
        if !indexes.contains_key(&i) {
            res.push(i)
        }
    }

    res
}