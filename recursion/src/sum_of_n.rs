pub mod sum_of_n {
    pub fn calculate(n  : isize) -> isize {
        return if n == 0 {
            0
        } else {
            calculate(n-1)+ n
        }
    }
}