use std::{collections::HashMap, rc::Rc, cell::RefCell};

struct Node {
    key: i32,
    next: Option<Rc<RefCell<Node>>>,
    prevs: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32) -> Self {
        Node {key: key, next: None, prevs: None}
    }
}

struct LRUCache {
    capacity: usize,
    dic: HashMap<i32, (i32, Rc<RefCell<Node>>)>,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LRUCache {

    fn new(capacity: i32) -> Self {
        LRUCache {capacity: capacity as usize, dic: HashMap::new(), head: None, tail: None}
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some((value, node)) = self.dic.get(&key).cloned() {
            self.remove_node(node.clone());
            self.append_node(node.clone());
            value
        } else {
            -1
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some((_, node)) = self.dic.get(&key).cloned() {
            self.remove_node(node.clone());
            self.append_node(node.clone());
            self.dic.insert(key, (value, node.clone()));
        } else {
            if self.dic.len() >= self.capacity {
                let tail = self.tail.as_ref().unwrap().clone();
                self.dic.remove(&tail.borrow().key);
                self.remove_node(tail);
            }
            let node = Rc::new(RefCell::new(Node::new(key)));
            self.append_node(node.clone());
            self.dic.insert(key, (value, node.clone()));
        }
    }

    fn append_node(&mut self, node: Rc<RefCell<Node>>) {
        if let Some(head) = self.head.replace(node.clone()) {
            node.borrow_mut().next = Some(head.clone());
            head.borrow_mut().prevs = Some(node.clone());
        } else {
            self.tail = Some(node);
        }
    }

    fn remove_node(&mut self, node: Rc<RefCell<Node>>) {
        let prevs = node.borrow_mut().prevs.take();
        let next = node.borrow_mut().next.take();
        match (prevs, next) {
            (Some(prevs), Some(next)) => {
                prevs.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prevs = Some(prevs);    
            },
            (None, Some(next)) => {
                next.borrow_mut().prevs = None;
                self.head = Some(next);    
            },
            (Some(prevs), None) => {
                prevs.borrow_mut().next = None;
                self.tail = Some(prevs);    
            },
            (None, None) => {
                self.head = None;
                self.tail = None;    
            },
        }
    }
}

mod lrucache_test {
    use super::*;

    #[test]
    fn test_1() {
        let mut lrucache = LRUCache::new(2);
        lrucache.put(1, 1);
        lrucache.put(2, 2);
        assert_eq!(lrucache.get(1), 1);
        lrucache.put(3, 3);
        assert_eq!(lrucache.get(2), -1);
        lrucache.put(4, 4);
        assert_eq!(lrucache.get(1), -1);
        assert_eq!(lrucache.get(3), 3);
        assert_eq!(lrucache.get(4), 4);
    }    
}    