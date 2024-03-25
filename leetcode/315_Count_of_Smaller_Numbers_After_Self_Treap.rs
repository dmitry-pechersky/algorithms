#[derive(Debug)]
struct Node { x: i32, y: u32, cnt: u32, total_cnt: u32, left: Option<u32>, right: Option<u32> }

impl Node {
    fn new(x: i32, y: u32) -> Self {
        Node { x, y, cnt: 1, total_cnt: 1, left: None, right: None }
    }
}

#[derive(Debug)]
pub struct Treap { nodes: Vec<Node>, root: Option<u32> }

impl Treap {
    pub fn with_capacity(capacity: u32) -> Self {
        Treap { nodes: Vec::with_capacity(capacity as usize), root: None }
    }

    fn smaller_cnt(&self, k: i32) -> u32 {
        Self::smaller_cnt_rec(&self.nodes, self.root, k)
    }

    fn smaller_cnt_rec(nodes: &Vec<Node>, node: Option<u32>, k: i32) -> u32 {
        let mut cnt = 0;
        if let Some(node) = node {
            if k < nodes[node as usize].x {
                cnt += Self::smaller_cnt_rec(nodes, nodes[node as usize].left, k)
            } else {
                cnt += if let Some(left) = nodes[node as usize].left { nodes[left as usize].total_cnt } else { 0 };
                if k > nodes[node as usize].x {
                    cnt += nodes[node as usize].cnt;
                    cnt += Self::smaller_cnt_rec(nodes, nodes[node as usize].right, k);
                }
            }
        } 
        cnt
    }

    pub fn push(&mut self, x: i32, y: u32) {
        let (left, right) = Self::split(&mut self.nodes, self.root, x);
        let (left, mut node) = Self::split(&mut self.nodes, left, x - 1);
        if let Some(node) = node {
            self.nodes[node as usize].cnt += 1;
            self.nodes[node as usize].total_cnt += 1;
        } else {
            self.nodes.push(Node::new(x, y));
            node = Some(self.nodes.len() as u32 - 1);
        }
        let left = Self::merge(&mut self.nodes, left, node);
        self.root = Self::merge(&mut self.nodes, left, right);
    }

    fn update_total_cnt(nodes: &mut Vec<Node>, node: u32) {
        let (left, right) = (nodes[node as usize].left, nodes[node as usize].right);
        let left_total_cnt = left.map_or(0, |left| nodes[left as usize].total_cnt);
        let right_total_cnt = right.map_or(0, |right| nodes[right as usize].total_cnt);
        nodes[node as usize].total_cnt = nodes[node as usize].cnt + left_total_cnt + right_total_cnt;
    }

    fn merge(nodes: &mut Vec<Node>, left: Option<u32>, right: Option<u32>) -> Option<u32> {
        match (left, right) {
            (None, right) => right,
            (left, None) => left,
            (Some(left), Some(right)) => {
                if nodes[left as usize].y < nodes[right as usize].y {
                    nodes[right as usize].left = Self::merge(nodes, Some(left), nodes[right as usize].left);
                    Self::update_total_cnt(nodes, right);
                    Some(right)
                } else {
                    nodes[left as usize].right = Self::merge(nodes, nodes[left as usize].right, Some(right));
                    Self::update_total_cnt(nodes, left);
                    Some(left)
                }
            }
        }
    }

    fn split(nodes: &mut Vec<Node>, node: Option<u32>, k: i32) -> (Option<u32>, Option<u32>) {
        if let Some(node) = node {
            if nodes[node as usize].x <= k {
                let (right_left, right_right) = Self::split(nodes, nodes[node as usize].right, k);
                nodes[node as usize].right = right_left;
                Self::update_total_cnt(nodes, node);
                (Some(node), right_right)
            } else {
                let (left_left, left_right) = Self::split(nodes, nodes[node as usize].left, k);
                nodes[node as usize].left = left_right;
                Self::update_total_cnt(nodes, node);
                (left_left, Some(node))
            }
        } else {
            (None, None)
        }
    }
}

struct Solution {}

impl Solution {
    pub fn count_smaller(mut nums: Vec<i32>) -> Vec<i32> {
        const A: u32 = 7;
        const B: u32 = 17467;
        let n = nums.len();
        let mut treap = Treap::with_capacity(n as u32);
        let mut y = 0;
        for i in (0..n).rev() {
            let cnt = treap.smaller_cnt(nums[i]) as i32;
            y = (y * A + B) % n as u32;
            treap.push(nums[i], y);
            nums[i] = cnt;
        }
        nums
    }
}

#[test]
fn test_1() {
    
    assert_eq!(vec![2,1,1,0], Solution::count_smaller(vec![5,2,6,1]));
    assert_eq!(vec![0], Solution::count_smaller(vec![-1]));
    assert_eq!(vec![0, 0], Solution::count_smaller(vec![-1, -1]));
    assert_eq!(vec![0,0,0,0], Solution::count_smaller(vec![5,6,7,8]));
    assert_eq!(vec![2,1,0], Solution::count_smaller(vec![1,0,-1]));
    assert_eq!(vec![2,4,4,2,2,0,0], Solution::count_smaller(vec![0,1,1,0,0,-1,-1]));
}