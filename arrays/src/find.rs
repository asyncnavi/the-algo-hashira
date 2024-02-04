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

pub fn duplicate(a : &[isize]) -> Vec<isize> {
    let mut result = Vec::new();
    let mut map_of_rep = HashMap::new();
    for &element in a {
        *map_of_rep.entry(element).or_insert(0) +=1;
    }
    for (k , &v) in &map_of_rep {
        if v > 1 {
            result.push(*k);
        }
    }

    result
}

pub fn pair_of_sum(a : &[isize], k : isize) -> Vec<isize> {
    let mut result = Vec::with_capacity(2);
    let mut map_of_diff = HashMap::new();
    for &element in a {
        let difference = k - element;

        // Check if the difference is already in the HashMap
        if map_of_diff.contains_key(&difference) {
            result.push(element);
            result.push(difference);
            return result;
        } else {
            map_of_diff.entry(element).or_insert(());
        }
    }

    result
}

pub fn pair_of_sum_sorted(a : &[isize], k : isize) -> Vec<isize> {
    let mut result = Vec::with_capacity(2);
    let mut i = 0;
    let mut j = a.len();

    while i < j {
        if (a[i] + a[j]) == k {
            result.push(i as isize);
            result.push(j as isize);
            return result;
        } else if (a[i] + a[j]) > k {
            j-=1;
        } else {
            i+1;
        }
    }

    result
}