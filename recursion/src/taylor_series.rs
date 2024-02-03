pub mod taylor_series {
    pub unsafe fn sum(x : isize, n : isize) -> f64 {
            static mut P: f64 = 1.0;
            static mut F: f64 = 1.0;

        return if n == 0 {
            1.0
        } else {
            let e = sum(x, n - 1);
            P = P * x  as f64;
            F = F * n as f64;

            e + P / F
        }

    }
    pub unsafe fn sum_in_linear_time(x : isize, n : isize) -> f64 {
        static mut S : f64 = 1.0;

        return if n == 0 {
            S
        } else {
            S = 1.0 + (x as f64 * S) / n as f64;
            sum_in_linear_time(x, n - 1)
        }
    }
}