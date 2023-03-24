struct Node {
    children: [Option<Box<Node>>; Self::CHAR_N],
    end: bool,
}

impl Node {
    const CHAR_N: usize = 26;
    const CHAR_SHIFT: u8 = b'a';

    fn new() -> Self {
        const INIT: Option<Box<Node>> = None; 
        Self { children: [INIT; Node::CHAR_N], end: false }
    }
}

struct WordDictionary {
    root: Box<Node>,
}

impl WordDictionary {
    fn new() -> Self {
        Self { root: Box::new(Node::new()) }
    }
    
    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.as_bytes().iter().map(|v| v - Node::CHAR_SHIFT) {
            if node.children[ch as usize].is_none() {
                node.children[ch as usize] = Some(Box::new(Node::new()));
            }
            node = node.children[ch as usize].as_mut().unwrap();
        }
        node.end = true;
    }
    
    fn search(&self, word: String) -> bool {
        fn rec(node: &Box<Node>, word: &[u8], i: usize) -> bool {
            if i == word.len() {
                node.end
            } else {
                if word[i] == b'.' {
                    for next_node in &node.children {
                        if let Some(next_node) = next_node.as_ref() {
                            if rec(next_node, word, i + 1) {
                                return true
                            }
                        }
                    }
                    false
                } else if let Some(next_node)  = node.children[(word[i] - Node::CHAR_SHIFT) as usize].as_ref() {
                    rec(next_node, word, i + 1)
                } else {
                    false
                }
            } 
        }
        rec(&self.root, word.as_bytes(), 0)
    }
}


#[test]
fn test_1() {
    let mut word_dictionary = WordDictionary::new();
    word_dictionary.add_word("bad".to_string());
    word_dictionary.add_word("dad".to_string());
    word_dictionary.add_word("mad".to_string());
    assert!(!word_dictionary.search("pad".to_string())); 
    assert!(word_dictionary.search("bad".to_string())); 
    assert!(word_dictionary.search(".ad".to_string())); 
    assert!(word_dictionary.search("b..".to_string())); 
}    

