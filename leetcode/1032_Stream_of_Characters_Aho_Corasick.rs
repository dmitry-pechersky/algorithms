use std::collections::VecDeque;

const ALTH_N: usize = 26;
const ALTH_SHIFT: u8 = b'a';

#[derive(Debug)]
struct Node {
    terminal: bool,
    childs: [Option<u32>; ALTH_N]
}

impl Default for Node {
    fn default() -> Self {
        Self { terminal: false, childs: [None; ALTH_N] }
    }
}


struct StreamChecker {
    to: Vec<[u32; ALTH_N]>,
    comp: Vec<u32>,
    trie: Vec<Node>,
    v: u32,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let trie = Self::trie(words);
        let n = trie.len();
        let (mut to, mut link, mut comp) = (vec![[0; ALTH_N]; n], vec![0; n], vec![0; n]);
        let mut queue = VecDeque::new();
        for (ch, u) in trie[0].childs.iter().enumerate() {
            if let &Some(u) = u {
                to[0][ch as usize] = u;
                queue.push_back(u);
            }
        }
        while let Some(v) = queue.pop_front() {
            for (ch, u) in trie[v as usize].childs.iter().enumerate() {
                if let &Some(u) = u {
                    to[v as usize][ch as usize] = u;                    
                    link[u as usize] = to[link[v as usize] as usize][ch as usize];
                    queue.push_back(u);
                } else {
                    to[v as usize][ch as usize] = to[link[v as usize] as usize][ch as usize];
                }
            }
            if trie[link[v as usize] as usize].terminal {
                comp[v as usize] = link[v as usize];
            } else {
                comp[v as usize] = comp[link[v as usize] as usize];
            }
        }
        Self {to, comp , trie, v: 0}
    }
    
    fn query(&mut self, letter: char) -> bool {
        self.v = self.to[self.v as usize][(letter as u8 - ALTH_SHIFT) as usize];
        self.trie[self.v as usize].terminal || self.comp[self.v as usize] != 0
    }

    fn trie(words: Vec<String>) -> Vec<Node> {
        let mut trie = vec![Node::default()];
        for word in words {
            let mut current = 0;
            for ch in word.as_bytes() {
                let ch = ch - ALTH_SHIFT;
                if trie[current].childs[ch as usize].is_none() {
                    trie.push(Node::default());
                    trie[current].childs[ch as usize] = Some(trie.len() as u32 - 1);
                }
                current = trie[current].childs[ch as usize].unwrap() as usize;
            }
            trie[current].terminal = true;
        }
        trie
    }
}

#[test]
fn test_stream_checker() {    
    let mut checker = StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
    assert!(!checker.query('a'));
    assert!(!checker.query('b'));
    assert!(!checker.query('c'));
    assert!(checker.query('d'));
    assert!(!checker.query('e')); 
    assert!(checker.query('f'));
    assert!(!checker.query('g'));
    assert!(!checker.query('h'));
    assert!(!checker.query('i'));
    assert!(!checker.query('j'));
    assert!(!checker.query('k'));
    assert!(checker.query('l'));    

    let mut checker = StreamChecker::new(vec!["ab".to_string(),"ba".to_string(),"aaab".to_string(),"abab".to_string(),"baa".to_string()]);
    assert!(!checker.query('a'));
    assert!(!checker.query('a'));
    assert!(!checker.query('a'));
    assert!(!checker.query('a'));
    assert!(!checker.query('a'));
    assert!(checker.query('b'));
    assert!(checker.query('a'));
    assert!(checker.query('b'));
    assert!(checker.query('a'));
    assert!(checker.query('b'));
    assert!(!checker.query('b'));
    assert!(!checker.query('b'));
    assert!(checker.query('a'));
    assert!(checker.query('b'));
    assert!(checker.query('a'));
    assert!(checker.query('b'));
    assert!(!checker.query('b'));
    assert!(!checker.query('b'));
    assert!(!checker.query('b'));
}
