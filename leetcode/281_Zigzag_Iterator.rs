struct ZigzagIterator {
    v1: Vec<i32>,
    v2: Vec<i32>,
    i1: usize,
    i2: usize,
    first: bool,
}

impl ZigzagIterator {
    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        let first = !v1.is_empty();
        Self { v1, v2, i1: 0, i2: 0, first}
    }
    
    fn next(&mut self) -> i32 {
        let mut res;
        if self.first {
            res = self.v1[self.i1];
            self.i1 += 1;
            if self.i2 < self.v2.len() {
                self.first = ! self.first;
            }
        } else {
            res = self.v2[self.i2];
            self.i2 += 1;
            if self.i1 < self.v1.len() {
                self.first = ! self.first;
            }
        }
        res
    }
    
    fn has_next(&self) -> bool {
        self.i1 < self.v1.len() || self.i2 < self.v2.len()
    }
}
