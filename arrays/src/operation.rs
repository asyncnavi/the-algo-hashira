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
}

