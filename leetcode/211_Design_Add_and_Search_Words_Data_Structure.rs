#[derive(Debug)]
struct Node {
    array: [Option<Box<Node>>; 26],
    end: bool,
}

struct WordDictionary {
    root: Box<Node>,

}

impl Node {
    fn new() -> Self {
        const INIT: Option<Box<Node>> = None; 
        Node {
            array: [INIT; 26],
            end: false,
        }
    }
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: Box::new(Node::new()),
        }
    }
    
    fn add_word(&mut self, word: String) {
        let mut current = &mut self.root;
        for ch in word.as_bytes(){
            current = current.array[(*ch - b'a') as usize].get_or_insert(Box::new(Node::new()));
        }
        current.end = true;        
    }
    
    fn search(&self, word: String) -> bool {
        self._search_rec(&self.root, word.as_bytes(), 0)
    }

    fn _search_rec(&self, node: &Box<Node>, word: &[u8], i: usize) -> bool {
        if i < word.len() {
            let ch = word[i];
            if ch == b'.' {
                for option in node.array.iter() {
                    if option.is_some() && self._search_rec(option.as_ref().unwrap(), word, i + 1) {
                        return true;
                    } 
                }
                return false
            } else {
                let option = node.array[(ch - b'a') as usize].as_ref();
                return option.is_some() && self._search_rec(option.unwrap(), word, i + 1);
            }
        }
        node.end
    }
}



#[cfg(test)]
mod word_dictionary_test {
    use super::*;

    #[test]
    fn test_1() {
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word("bad".to_string());
        word_dictionary.add_word("dad".to_string());
        word_dictionary.add_word("mad".to_string());
        assert!(! word_dictionary.search("pad".to_string())); 
        assert!(word_dictionary.search("bad".to_string())); 
        assert!(word_dictionary.search(".ad".to_string())); 
        assert!(word_dictionary.search("b..".to_string())); 
    }    
}
