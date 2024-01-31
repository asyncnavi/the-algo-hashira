pub mod operation {
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

}