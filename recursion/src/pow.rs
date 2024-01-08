pub mod pow {
    pub fn calculate(m : isize, n : isize) -> isize {
        return if n == 0 {
            1
        } else {
            calculate(m, n -1) * m
        }
    }
}