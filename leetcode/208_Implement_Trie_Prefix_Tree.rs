
struct Node {
    children: [Option<Box<Node>>; Trie::CHAR_N],
    terminal: bool,
}

impl Node {
    fn new() -> Self {
        const INIT: Option<Box<Node>> = None;
        Self { children: [INIT; Trie::CHAR_N], terminal: false}
    }
}

struct Trie {
    root: Box<Node>,
}

impl Trie {

    const CHAR_N: usize = 26;
    const CHAR_SHIFT: u8 = b'a';

    fn new() -> Self {
        Self { root: Box::new(Node::new()) }
    }
    
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.as_bytes().iter().map(|v| v - Trie::CHAR_SHIFT) {
            if node.children[ch as usize].is_none() {
                node.children[ch as usize] = Some(Box::new(Node::new()));
            }
            node = node.children[ch as usize].as_mut().unwrap();
        }
        node.terminal = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for ch in word.as_bytes().iter().map(|v| v - Trie::CHAR_SHIFT) {
            match &node.children[ch as usize] {
                Some(child) => { node = child },
                None => { return false}
            } 
        }
        node.terminal
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for ch in prefix.as_bytes().iter().map(|v| v - Trie::CHAR_SHIFT) {
            match &node.children[ch as usize] {
                Some(child) => { node = child },
                None => { return false }
            } 
        }
        true        
    }
}

#[test]
fn test_1() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert!(trie.search("apple".to_string()));
    assert!(!trie.search("app".to_string()));
    assert!(trie.starts_with("app".to_string()));
    trie.insert("app".to_string());
    assert!(trie.search("app".to_string()));
}    

