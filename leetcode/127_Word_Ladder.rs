use std::collections::{HashMap, VecDeque};

struct Solution {}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        fn find_target(target_word: &Vec<u8>, word_list: &Vec<Vec<u8>>) -> Option<usize> {
            word_list.iter().enumerate().find(|(_, s)| *s == target_word).map(|(i, _)| i)
        }

        fn create_pattern_dic(word_n: usize, word_list: &Vec<Vec<u8>>) -> HashMap<Vec<u8>, Vec<usize>> {
            let mut dic: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();
            for (v, word) in word_list.iter().enumerate() {
                for i in 0..word_n {
                    let mut pattern = word.clone();
                    pattern[i] = b'*';
                    if let Some(list) = dic.get_mut(&pattern) {
                        list.push(v);
                    } else {
                        dic.insert(pattern, vec![v]);
                    }
                }
            }
            dic    
        }

        fn find_starts(start_word: &Vec<u8>, word_list: &Vec<Vec<u8>>, pattern_dic: &HashMap<Vec<u8>, Vec<usize>>) -> VecDeque<(usize, i32)> {
            let mut starts = VecDeque::new();

            for (idx, word) in word_list.iter().enumerate() {
                if start_word == word {
                    starts.push_back((idx, 1));
                    break;
                }
            }

            let mut pattern = start_word.clone();
            for i in 0..start_word.len() {
                let ch = pattern[i];
                pattern[i] = b'*';
                if let Some(list) = pattern_dic.get(&pattern) {
                    for &v in list {
                        starts.push_back((v, 2));
                    }
                }
                pattern[i] = ch;
            }

            starts
        }

        fn create_adj_list(n: usize, pattern_dic: &HashMap<Vec<u8>, Vec<usize>>) -> Vec<Vec<usize>> {
            let mut adj_list = vec![vec![]; n];
            for list in pattern_dic.values() {
                let list_n = list.len();
                for i in 0..list_n {
                    for j in i + 1..list_n {
                        adj_list[list[i]].push(list[j]);
                        adj_list[list[j]].push(list[i]);
                    }
                }
            }
            adj_list
        }

        let (begin_word, end_word) = (begin_word.into_bytes(), end_word.into_bytes());
        let word_list: Vec<Vec<u8>> = word_list.into_iter().map(|w| w.into_bytes()).collect();
        let (n, word_n) = (word_list.len(), begin_word.len());

        if let Some(target) = find_target(&end_word, &word_list) { 
            let pattern_dic = create_pattern_dic(word_n, &word_list);
            let mut queue = find_starts(&begin_word, &word_list, &pattern_dic);
            let adj_list = create_adj_list(n, &pattern_dic);
            let mut expanded = vec![false; n];
    
            while let Some((v, cnt)) = queue.pop_front() {
                if v == target {
                    return cnt;
                }
                if !expanded[v] {
                    expanded[v] = true;
                    for &u in &adj_list[v] {
                        queue.push_back((u, cnt + 1));
                    }
                }
            }
        };
        0
    }
}

#[test]
fn test_1() {
    assert_eq!(
        5, 
        Solution::ladder_length(
            "hit".to_string(), 
            "cog".to_string(),
            ["hot","dot","dog","lot","log","cog"].iter().map(|s| s.to_string()).collect()
        )
    );
}

#[test]
fn test_2() {
    assert_eq!(
        0, 
        Solution::ladder_length(
            "hit".to_string(), 
            "cog".to_string(),
            ["hot","dot","dog","lot","log"].iter().map(|s| s.to_string()).collect()
        )
    );
}
