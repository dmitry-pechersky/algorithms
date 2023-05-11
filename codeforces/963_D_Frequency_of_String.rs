use std::{io::{self, Write}, collections::VecDeque};

const ALPHABET_SIZE: usize = 26;

#[derive(Debug)]
struct Node {
    childs: [u32; ALPHABET_SIZE],
    is_terminal: bool,
    string_idx: u32,
}

impl Node {
    fn new() -> Self {
        Node { childs: [0; ALPHABET_SIZE], is_terminal: false, string_idx: 0 }
    }
}

struct AhoCorasick {
    to: Vec<Vec<u32>>,
    comp: Vec<u32>,
    trie: Vec<Node>,
    pattern_cnt: u32
}

impl AhoCorasick {
    fn new(patterns: &[String]) -> Self {
        let trie = Self::build_trie(patterns);
        let (to, comp) = Self::build_state_machine(&trie);
        Self { to, comp, trie, pattern_cnt: patterns.len() as u32 }
    }

    fn find(&self, string: &str) -> Vec<Vec<u32>> {
        let mut v = 0;
        let mut occurrences = vec![vec![]; self.pattern_cnt as usize];
        for (idx, ch) in string.as_bytes().iter().map(|ch| *ch - b'a').enumerate() {
            v = self.to[v as usize][ch as usize];
            let mut u = v; 
            while u != 0 {
                if self.trie[u as usize].is_terminal {
                    occurrences[self.trie[u as usize].string_idx as usize].push(idx as u32);
                }
                u = self.comp[u as usize];
            }
        }
        occurrences
    }

    fn build_trie(strings: &[String]) -> Vec<Node> {
        let mut trie = vec![Node::new()];
    
        for (idx, string) in strings.iter().enumerate() {
            let mut current = 0;
            for ch in string.as_bytes().iter().map(|ch| *ch - b'a') {
                if trie[current].childs[ch as usize] == 0 {
                    trie.push(Node::new());
                    trie[current].childs[ch as usize] = trie.len() as u32 - 1;
                }
                current = trie[current].childs[ch as usize] as usize;
            }
            trie[current].is_terminal = true;
            trie[current].string_idx = idx as u32;
        }
        trie
    }

    fn build_state_machine(trie: &Vec<Node>) -> (Vec<Vec<u32>>, Vec<u32>) {
        let n = trie.len();
        let mut to = vec![vec![0;ALPHABET_SIZE]; n];
        let mut link = vec![0; n];
        let mut comp = vec![0; n];

        let mut queue = VecDeque::new();
        for ch in 0..ALPHABET_SIZE {
            if trie[0].childs[ch] != 0 {
                let v = trie[0].childs[ch];
                to[0][ch] = v;
                queue.push_back(v);    
            }
        }

        while let Some(v) = queue.pop_front() {
            for ch in 0..ALPHABET_SIZE {
                if trie[v as usize].childs[ch] == 0 {
                    to[v as usize][ch] = to[link[v as usize] as usize][ch];
                } else {
                    let u = trie[v as usize].childs[ch];
                    to[v as usize][ch] = u;
                    link[u as usize] = to[link[v as usize] as usize][ch];
                    comp[u as usize] = if trie[link[u as usize] as usize].is_terminal { link[u as usize] } else { comp[link[u as usize] as usize] };
                    queue.push_back(u);
                }
            }
        }
        (to, comp)
    }
    
}

fn frequency_of_string(string: &str, patterns: &[String], cnts: &[u32]) -> Vec<i32> {
    let occurrences = AhoCorasick::new(&patterns).find(&string);
    let mut res = vec![];

    for ((occurrences, pattern), &cnt) in occurrences.into_iter().zip(patterns.iter()).zip(cnts) {
        let n = pattern.len();
        if occurrences.len() < cnt as usize {
            res.push(-1);
        } else {
            let mut min_length = u32::MAX;
            for i in 0..occurrences.len() - cnt as usize + 1 {
                let length = occurrences[i + cnt as usize - 1]  + 1 - (occurrences[i]  + 1 - n as u32);
                min_length = min_length.min(length);
         
            }
            res.push(min_length as i32);
        }
    }
    res
}

fn input() -> impl Iterator<Item=String> {
    io::stdin()
        .lines()
        .map(
            |line| 
            line.unwrap()
                .split_ascii_whitespace()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
    ).flatten()
}

fn main() {
    let mut iter = input();

    let string = iter.next().unwrap();
    let n: u32 = iter.next().unwrap().parse().unwrap();

    let mut patterns = vec![];
    let mut cnts = vec![];

    for _ in 0..n {
        cnts.push(iter.next().unwrap().parse::<u32>().unwrap());
        patterns.push(iter.next().unwrap());
    }

    let mut out = io::stdout().lock();
    for res in frequency_of_string(&string, &patterns, &cnts) {
        writeln!(&mut out, "{}", res).unwrap();
    }

}

#[test]
fn test_aho_corasick() {
    let aho = AhoCorasick::new(&["a".to_string(), "ab".to_string(), "cab".to_string()]);
    
    assert_eq!(
        vec![vec![0, 3], vec![1, 4], vec![4]],
        aho.find("abcabc")
    );
    
    let aho = AhoCorasick::new(&["a", "b", "c", "ab", "ca", "abc"].iter().map(|s| s.to_string()).collect::<Vec<_>>());
    
    assert_eq!(
        vec![vec![0,3,5],vec![1,6],vec![2,4,7], vec![1,6], vec![3,5], vec![2,7]],
        aho.find("abcacabc")
    );
}

#[test]
fn test_frequency_of_string() {
    assert_eq!(
        vec![3, 4, 4, -1, 5],
        frequency_of_string(
            "aaaaa", 
            &["a", "aa", "aaa", "aaaa", "aaaaa"].iter().map(|s| s.to_string()).collect::<Vec<_>>(), 
            &[3, 3, 2, 3, 1]
        )
    );
}