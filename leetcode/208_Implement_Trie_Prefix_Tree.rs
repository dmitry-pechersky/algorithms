struct Node {
    alp: [Option<Box<Node>>; 26],
    is_word: bool,
}

struct Trie {
    root: Box<Node>,
}

impl Node {
    fn new() -> Self {
        const INIT: Option<Box<Node>> = None;
        Node {alp: [INIT; 26], is_word: false}
    }
}

impl Trie {
    fn new() -> Self {
        Trie { root: Box::new(Node::new()) }
    }
    
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for &ch in word.as_bytes() {
            node = node.alp[(ch - b'a') as usize].get_or_insert(Box::new(Node::new()));
        }
        node.is_word = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for &ch in word.as_bytes() {
            match &node.alp[(ch - b'a') as usize] {
                Some(v) => { node = v },
                None => { return false },
            }
        }
        node.is_word
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for &ch in prefix.as_bytes() {
            match &node.alp[(ch - b'a') as usize] {
                Some(v) => { node = v },
                None => { return false },
            }
        }
        true
    }
}

#[cfg(test)]
mod trie_test {
    use super::*;

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
}
