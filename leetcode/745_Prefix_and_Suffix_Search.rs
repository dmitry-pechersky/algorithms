struct Node {
    chars: [Option<Box<Node>>; 27],
    max_idx: i16,
}

impl Node {
    fn new() -> Self {
        Self { chars: Default::default(), max_idx: -1 }
    }
}

struct WordFilter {
    root: Box<Node>,
}

impl WordFilter {

    fn new(words: Vec<String>) -> Self {
        let mut root = Box::new(Node::new());
        for (idx, word) in words.into_iter().enumerate() {
            Self::append_word(&mut root, word.as_bytes(), idx as i16);
        }
        Self { root }
    }
    
    fn f(&self, prefix: String, suffix: String) -> i32 {
        let prefix = prefix.as_bytes();
        let suffix = suffix.as_bytes();
        let mut node = &self.root;
        for ch in [suffix, &[b'{'], prefix].concat() {
            let ch = ch - b'a';
            if node.chars[ch as usize].is_none() {
                return -1;
            }
            node = node.chars[ch as usize].as_ref().unwrap();
        }
        node.max_idx as i32
    }

    fn append_word(root: &mut Box<Node>, word: &[u8], idx: i16) {
        let n = word.len();
        let word = [word, &[b'{'], word].concat();
        for i in 0..=n {
            let mut node = &mut *root;
            for ch in &word[i..] {
                let ch = ch - b'a';
                if node.chars[ch as usize].is_none() {
                    node.chars[ch as usize] = Some(Box::new(Node::new()));
                }
                node = node.chars[ch as usize].as_mut().unwrap();
                node.max_idx = idx;
                    
            }
        }
    }
}

#[test]
fn test_1() {
    let word_filter = WordFilter::new(vec!["apple".to_string()]);
    assert_eq!(0, word_filter.f("a".to_string(), "e".to_string()));
}
