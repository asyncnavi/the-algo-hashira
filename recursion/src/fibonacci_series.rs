pub mod fibonacci_series {
    pub unsafe fn sum(n : i64) -> i64 {
        static mut S : i64 = 0;
        return if n <= 1 {
           n
        } else {
            sum(n-1) + sum(n-2)
        }
    }

    pub fn iterative_sum(n : i64) -> i64 {

        return if n == 0 {
            0
        } else if n == 1 {
            1
        }  else {
            let mut term_1: i64 = 0;
            let mut term_2 : i64 = 1;
            let mut next_term = 0;

            for i in 2..n {
                next_term = term_1 + term_2;
                term_1 = term_2;
                term_2 = next_term;
            }
            next_term
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
