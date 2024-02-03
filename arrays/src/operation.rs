pub mod operation {
    use std::mem::swap;

    pub fn sum(arr : &[isize]) -> isize {
        let mut sum = 0;
        for i in  0..arr.len() {
            sum += arr[i];
        }
        sum
    }

    pub fn max(arr : &[isize]) -> isize {
       let mut max =  arr[0];
       for i in 1..arr.len() {
            if (arr[i] > max) {
                 max = arr[i];
            }
       }

        max
    }

    pub fn avg(arr : &[isize]) -> isize {
        let mut total  = 0;
        for i in 0..arr.len() {
            total += arr[i];
        }

        total / arr.len() as isize
    }

    pub fn rev<T>(arr: &mut [T]) {
        let len = arr.len();
        let mid = len /2 ;

        for i in 0..mid {
            arr.swap(i, len - 1 -i);
        }
    }

    pub fn rev_with_copy<T: Copy>(arr: &mut [T]) {
        let mut help_arr: Vec<T> = Vec::with_capacity(arr.len());

        for i in (0..arr.len()).rev() {
            help_arr.push(arr[i]);
        }

        for i in 0..arr.len() {
            arr[i] = help_arr[i];
        }
    }

    pub fn is_sorted(arr :  &[isize]) -> bool {
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i+1] {
                return false;
            }
        }

        true
    }


    pub fn shift_signs(arr : &mut [isize])  {
        let mut i = 0;
        let mut j = arr.len() - 1;

        while i < j {
            while arr[i] < 0 {
                i += 1;
            }
            while arr[j] > 0 {
                j -= 1;
            }

            if(i < j) {
                arr.swap(i,j);
            }
        }
    }

    pub fn merge_sorted(a : &[isize], b : &[isize] ) -> Vec<isize>{
        // creating new arrays
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
            } else {
                c.push(b[j]);
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
 }
