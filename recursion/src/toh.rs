pub mod toh {
    pub fn tower_of_hanoi(n: u32, from_rod: char, to_rod: char, aux_rod: char) {
        if n == 1 {
            println!("Move disk 1 from rod {} to rod {}", from_rod, to_rod);
            return;
        }

        tower_of_hanoi(n - 1, from_rod, aux_rod, to_rod);
        println!("Move disk {} from rod {} to rod {}", n, from_rod, to_rod);
        tower_of_hanoi(n - 1, aux_rod, to_rod, from_rod);
    }
}