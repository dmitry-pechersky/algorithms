use std::collections::BinaryHeap;
use core::cmp::Reverse;

struct DinnerPlates {
    capacity: usize,
    stacks: Vec<Vec<i32>>,
    largest_non_empty: isize,
    free_stacks: BinaryHeap<isize>,
}

impl DinnerPlates {

    fn new(capacity: i32) -> Self {
        Self { capacity: capacity as usize, stacks: Vec::new(), largest_non_empty: -1, free_stacks: BinaryHeap::new() }
    }
    
    fn push(&mut self, val: i32) {
        if let Some(i) = self.free_stacks.peek() {
            let i = - i as usize;
            self.stacks[i].push(val);
            if self.stacks[i].len() == self.capacity {
                self.free_stacks.pop();
            }
            self.largest_non_empty = self.largest_non_empty.max(i as isize);
        } else {
            self.stacks.push(vec![val]);
            if self.capacity > 1 {
                self.free_stacks.push(- (self.stacks.len() as isize - 1));
            }
            self.largest_non_empty = self.stacks.len() as isize - 1;
        }
        
    }
    
    fn pop(&mut self) -> i32 {
        while self.largest_non_empty >= 0 {
            if let Some(value) =  self.stacks[self.largest_non_empty as usize].pop() {
                if self.stacks[self.largest_non_empty as usize].len() == self.capacity - 1 {
                    self.free_stacks.push(- self.largest_non_empty);
                }
                return value as i32
            }
            self.largest_non_empty -= 1;
        }
        -1
    }
    
    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let index = index as usize;
        if index < self.stacks.len() {
            if let Some(value) = self.stacks[index].pop() {
                if self.stacks[index].len() == self.capacity - 1 {
                    self.free_stacks.push(- (index as isize));
                }
                return value;
            }
        } 
        -1
    }
}