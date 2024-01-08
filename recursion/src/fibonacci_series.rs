pub mod fibonacci_series {
    pub unsafe fn sum(n : i64) -> i64 {
        static mut S : i64 = 0;
        return if n <= 1 {
           n
        } else {
            sum(n-1) + sum(n-2);
        }
    }
}
// sum(5)
// /           \
// sum(4)           sum(3)
// /       \        /        \
// sum(3)    sum(2)   sum(2)    sum(1)
// /     \      /  \     /  \
// sum(2) sum(1) sum(1) sum(0) sum(1)
// /  \
// sum(1) sum(0)
