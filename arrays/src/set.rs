pub fn union_with_sort(a : &[isize], b : &[isize]) -> Vec<isize> {
    let new_len = a.len() + b.len();
    let mut c = Vec::with_capacity(new_len);

    let mut i = 0;
    let mut j = 0;
    let m = a.len();
    let n = b.len();


    while i < m && j < n {
        if(a[i] < b[j]) {
            c.push(a[i]);
            i+=1;
        } else if a[i] > b[j] {
            c.push(b[j]);
            j+=1;
        } else {
            c.push(a[i]);
            i+=1;
            j+=1;
        }
    }

    while i < m {
        c.push(a[i]);
        i+=1;
    }

    while j < n {
        c.push(b[j]);
        j+=1;
    }
    c
}

/// Union of two arrays
pub fn union(a : &[isize], b : &[isize]) -> Vec<isize> {
    let new_len = a.len() + b.len();
    let mut c = Vec::with_capacity(new_len);

    // copying all the element of first array

    for i in 0..a.len() {
        c.push(a[i]);
    }

    // searching duplicates in first array and then pushing in the second array

    for j in 0..b.len() {
        let mut check = false;
        for i in 0..a.len() {
            if(b[j] == a[i]) {
                check = true;
            }
        }

        if !check {
            c.push(b[j]);
        }

    }

    c
}

/// Intersect of two arrays
pub fn intersect(a : &[isize], b : &[isize]) -> Vec<isize> {
    let new_len = a.len() + b.len();
    let mut c = Vec::with_capacity(new_len);

    for i in 0..a.len() {
        for j in 0..b.len() {
            if a[i] == b[j] {
                c.push(a[i]);
            }
        }
    }
    c
}
/// Intersect of two sorted arrays
pub fn intersect_with_sort(a : &[isize], b : &[isize]) -> Vec<isize> {
    let new_len = a.len() + b.len();
    let mut c = Vec::with_capacity(new_len);

    let mut i = 0;
    let mut j = 0;
    let m = a.len();
    let n = b.len();


    while i < m && j < n {
        if(a[i] < b[j]) {
            i+=1;
        } else if a[i] > b[j] {
            j+=1;
        } else {
            c.push(a[i]);
            i+=1;
            j+=1;
        }
    }
    c
}
/// Difference of two  arrays
pub fn diff(a : &[isize], b : &[isize]) -> Vec<isize> {
    let new_len = a.len() + b.len();
    let mut c = Vec::with_capacity(new_len);

    for i in 0..a.len() {
        let mut check = false;
        for j in 0..b.len() {
            if a[i] == b[j] {
                check = true;
            }
        }

        if !check {
            c.push(a[i]);
        }
    }
    c
}
/// Difference of two sorted arrays

pub fn diff_with_sort(a : &[isize], b : &[isize]) -> Vec<isize> {
    let new_len = a.len() + b.len();

    let mut c = Vec::with_capacity(new_len);
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len()  {
        if a[i] < a[j] {
            c.push(a[i]);
            i+=1;
        } else if a[i] > a[j] {
            i+=1;
        } else {
            i+=1;
            j+=1;
        }
    }


    while i < a.len() {
        c.push(a[i]);
        i+=1;
    }

    c
}