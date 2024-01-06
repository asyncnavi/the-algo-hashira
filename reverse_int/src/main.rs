fn reverse(x: i32) -> i32 {
    let mut num = x;
    let mut rev = 0;

    while num != 0 {
        let digit = num % 10;
        num /= 10;

        if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && digit > 7) {
            return 0; // Return 0 for overflow
        } else if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && digit < -8) {
            return 0; // Return 0 for overflow
        }

        rev = rev * 10 + digit;
    }

    rev
}

fn main() {
    let num = 12345;
    println!("Reversed number of {} is {}", num, reverse(num));
}
