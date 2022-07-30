#[derive(Debug)]
struct Node {
    cnt: u32,
    zero: Option<Box<Node>>,
    one: Option<Box<Node>>,
}

impl Node {
    fn new() -> Self {
        Node { cnt: 0, zero: None, one: None }
    }

    fn insert(mut node: &mut Box<Node>, value: u32, n: u32) {
        for i in (0..n).rev() {
            let next_node = if 0 == value & (1 << i) { &mut node.zero } else { &mut node.one };
            if next_node.is_none() {
                next_node.replace(Box::new(Self::new()));
            }
            node = next_node.as_mut().unwrap();
            node.cnt += 1;
        }
    }    

    fn remove(mut node: &mut Box<Node>, value: u32, n: u32) {
        for i in (0..n).rev() {
            let next_node = if 0 == value & (1 << i) { &mut node.zero } else { &mut node.one };
            node = next_node.as_mut().unwrap();
            node.cnt -= 1;
        }
    }    

    fn max_xor(mut node: &Box<Node>, value: u32, n: u32) -> u32 {
        let mut max_value: u32 = 0;
        for i in (0..n).rev() {
            max_value = max_value << 1;
            node = match (value & (1 << i), node.zero.as_ref(), node.one.as_ref()) {
                (_, Some(zero), None) => { zero },
                (_, None, Some(one)) => { max_value += 1; one },
                (_, Some(zero), Some(one)) if zero.cnt > 0 && one.cnt == 0 => { zero },
                (_, Some(zero), Some(one)) if zero.cnt == 0 && one.cnt > 0 => { max_value += 1; one },
                (0, Some(_), Some(one)) => { max_value += 1; one },
                (1.., Some(zero), Some(_)) => { zero },
                (_, _, _) => { unreachable!() }
            }
        }
        value ^ max_value
    }
}


struct Solution {}

impl Solution {
    pub fn max_genetic_difference(parents: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = parents.len();
        let mut adj_list = vec![vec![]; n];
        let mut node_queries = vec![vec![]; n];
        let mut res = vec![0; queries.len()];
        let mut root = 0;

        for (u, v) in parents.into_iter().enumerate() {
            if v >= 0 {
                adj_list[v as usize].push(u);
            } else {
                root = u;
            }
        }

        for (i, query) in queries.iter().enumerate() {
            let node = query[0];
            node_queries[node as usize].push(i);
        }

        // DFS
        let mut stack = vec![root];
        let mut expanded = vec![false; n];
        let mut trie = Box::new(Node::new());
        const BIT_N: u32 = 17;
        while let Some(v) = stack.pop() {
            if expanded[v] {
                Node::remove(&mut trie, v as u32, BIT_N);
            } else {
                expanded[v] = true;
                Node::insert(&mut trie, v as u32, BIT_N);
                stack.push(v);
                for u in adj_list[v].iter() {
                    stack.push(*u);
                }

                for (i, value) in node_queries[v].iter().map(|i| (i, queries[*i][1] as u32)) {
                    res[*i] = Node::max_xor(&trie, value, BIT_N) as i32;
                }
            }
        }
        res
    }
}

#[test]
fn test_1() {
    assert_eq!(vec![2,3,7], Solution::max_genetic_difference(vec![-1,0,1,1], vec![vec![0,2], vec![3,2], vec![2,5]]));
}

#[test]
fn test_2() {
    assert_eq!(vec![6,14,7], Solution::max_genetic_difference(vec![3,7,-1,2,0,7,0,2], vec![vec![4,6], vec![1,15], vec![0,5]]));
}
