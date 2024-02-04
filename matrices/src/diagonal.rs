pub struct Diagonal {
    size: usize,
    a: Vec<isize>,
}

impl Diagonal {
    pub fn new(size: usize) -> Diagonal {
        Diagonal {
            size,
            a: vec![0; size],
        }
    }

    pub fn set(&mut self, i: usize, j: usize, x: isize) {
        if i == j && i < self.size {
            self.a[i] = x;
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<isize> {
        if i == j && i < self.size {
            Some(self.a[i])
        } else {
            None
        }
    }
}
