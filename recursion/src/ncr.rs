pub mod ncr {
     pub fn calc(n : i64, r : i64) -> i64 {
        return if r == 0 || r == n {
            1
        } else {
            calc(n-1,r-1) + calc(n-1,r)
        }
     }
}