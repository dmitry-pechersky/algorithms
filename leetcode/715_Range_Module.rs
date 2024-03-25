use std::{collections::BTreeMap, ops::Bound::Excluded};

pub struct RangeModule {
    ranges: BTreeMap<i32,i32>,
    intersects: Vec<(i32, i32)>
}

impl RangeModule {

    pub fn new() -> Self {
        Self { ranges: BTreeMap::new(), intersects: vec![] }
    }
    
    pub fn add_range(&mut self, left: i32, right: i32) {
        for (&range_right, &range_left) in self.ranges.range(left..i32::MAX) {
            if range_left <= right {
                self.intersects.push((range_left, range_right));
            } else {
                break;
            }
        }
        if let (Some(&(first_left, _)), Some(&(_, last_right))) = (self.intersects.first(), self.intersects.last()) {
            for (_, range_right) in &self.intersects {
                self.ranges.remove(range_right);
            }

            let (new_left, new_right) = (first_left.min(left), last_right.max(right));
            self.ranges.insert(new_right, new_left);
        } else {
            self.ranges.insert(right, left);
        }
        self.intersects.clear();
    }
    
    pub fn query_range(&self, left: i32, right: i32) -> bool {
        if let Some((&range_right, &range_left)) = self.ranges.range(left..i32::MAX).next() { 
            if range_left <= left && right <= range_right {
                return true
            }
        }
        false
    }
    
    pub fn remove_range(&mut self, left: i32, right: i32) {
        for (&range_right, &range_left) in self.ranges.range((Excluded(left), Excluded(i32::MAX))) {
            if range_left < right {
                self.intersects.push((range_left, range_right));
            } else {
                break;
            }
        }        
        if let (Some(&(first_left, _)), Some(&(_, last_right))) = (self.intersects.first(), self.intersects.last()) {
            for (_, range_right) in &self.intersects {
                self.ranges.remove(range_right);
            }

            if first_left < left {
                let (new_left, new_right) = (first_left, left);
                self.ranges.insert(new_right, new_left);
            }

            if right < last_right {
                let (new_left, new_right) = (right, last_right);
                self.ranges.insert(new_right, new_left);
            }
        }
        self.intersects.clear();
    }
}

#[test] 
fn test_range_module() {
    let mut module = RangeModule::new();
    assert!(!module.query_range(1, 3));

    module.add_range(1, 3);
    module.add_range(5,7);
    assert!(!module.query_range(2, 6));
    assert!(module.query_range(2, 3));

    module.add_range(2,6);
    assert!(module.query_range(2, 6));

    module.remove_range(2, 6);
    assert!(!module.query_range(2, 6));
}