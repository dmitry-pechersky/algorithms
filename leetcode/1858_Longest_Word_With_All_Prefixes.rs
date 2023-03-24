struct Node {
    children: [Option<Box<Node>>; Self::CHAR_N],
    end: bool,
}

impl  Node {
    const CHAR_N: usize = 26;
    const CHAR_SHIFT: u8 = b'a';    

    fn new() -> Self {
        const INIT: Option<Box<Node>> = None;
        Node { children: [INIT; Self::CHAR_N], end: false }
    }
}

struct Solution {}

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        fn add(mut node: &mut Node, word: String) {
            for ch in word.as_bytes().iter().map(|v| v - Node::CHAR_SHIFT) {
                if node.children[ch as usize].is_none() {
                    node.children[ch as usize] = Some(Box::new(Node::new()));
                }
                node = node.children[ch as usize].as_mut().unwrap();
            }
            node.end = true;
        }

        fn dfs(node: &Node, word: &mut Vec<u8>, length: u32, max_length: &mut u32, res: &mut Option<Vec<u8>>) {
            let mut deepest = true;
            for (ch, child) in node.children.iter().enumerate() {
                if let Some(child) = child {                    
                    if child.end {
                        deepest = false;
                        word.push(ch as u8);
                        dfs(child,  word, length + 1, max_length, res);
                        word.pop();    
                    }
                }
            }
            if deepest && length > *max_length {
                *max_length = length;
                *res = Some(word.clone());
            }
        }

        let mut node = Node::new();
        node.end = true;
        for word in words.into_iter() {
            add(&mut node, word);
        }

        let mut res = None;
        dfs(&node, &mut vec![], 0, &mut 0, &mut res);

        match res {
            Some(res) => res.into_iter().map(|v| (v + Node::CHAR_SHIFT) as char).collect(),
            None => String::new(),
        }        
    }
}

#[test]
fn test_1() {
    assert_eq!(
        "kiran".to_string(),
        Solution::longest_word(["k","ki","kir","kira", "kiran"].iter().map(|v| v.to_string()).collect())
    );
}

#[test]
fn test_2() {
    assert_eq!(
        "apple".to_string(),
        Solution::longest_word(["a", "banana", "app", "appl", "ap", "apply", "apple"].iter().map(|v| v.to_string()).collect())
    );
}

#[test]
fn test_3() {
    assert_eq!(
        "".to_string(),
        Solution::longest_word(["abc", "bc", "ab", "qwe"].iter().map(|v| v.to_string()).collect())
    );
}