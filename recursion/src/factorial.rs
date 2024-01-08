pub mod factorial {
    pub fn calculate(n : i64) -> i64  {
        return if n == 0 || n == 1 {
            1
        } else {
            calculate(n - 1) * n
        }
    }

}