struct MinStack {
    vector: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        Self { vector: vec![] }
    }
    
    fn push(&mut self, val: i32) {
        let min_value = if let Some(top) = self.vector.last() { top.1 } else { val };
        self.vector.push((val, min_value.min(val)));
    }
    
    fn pop(&mut self) {
        self.vector.pop();
    }
    
    fn top(&self) -> i32 {
        if let Some(top) = self.vector.last() { 
            return top.0 
        } 
        unreachable!()
    }
    
    fn get_min(&self) -> i32 {
        if let Some(top) = self.vector.last() { 
            return top.1 
        } 
        unreachable!()        
    }
}

#[test]
fn test_1() {
    let mut stack = MinStack::new();
    stack.push(-2);
    stack.push(0);
    stack.push(-3);
    assert_eq!(stack.get_min(), -3);
    stack.pop();
    assert_eq!(stack.top(), 0);
    assert_eq!(stack.get_min(), -2);
}
