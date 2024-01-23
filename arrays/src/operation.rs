pub mod operation {
    pub fn sum(arr : &[isize]) -> isize {
        let mut sum = 0;
        for i in  arr[..] {
            sum += arr[i];
        }
        sum
    }

    pub fn max(arr : &[isize]) -> isize {
       let mut max =  arr[0];
       for i in arr[1..] {
            if (arr[i] > max) {
                 max = arr[i];
            }
       }

        max
    }

    pub fn avg(arr : &[isize]) -> isize {
        let mut total  = 0;
        for i in arr[..] {
            total += arr[i];
        }

        total / arr.len() as isize
    }
}