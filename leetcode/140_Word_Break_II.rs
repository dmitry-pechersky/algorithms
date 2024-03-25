use std::collections::HashSet;

fn word_break_rec<'a>(s: &'a str, i: usize, dict: &HashSet<&str>, sentence: & mut Vec<&'a str>, sentences: &mut Vec<String>) {
    const MAX_LENGTH: usize = 10;
    if i == s.len() {
        sentences.push(sentence.join(" "));
        return;
    }
    for j in i..(i + MAX_LENGTH).min(s.len()) {
        let word = &s[i..=j];
        if dict.contains(word) {
            sentence.push(word);
            word_break_rec(s, j + 1, dict, sentence, sentences);
            sentence.pop();
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let dict = word_dict.iter().map(|s| s.as_str()).collect::<HashSet<_>>();
        let mut res = vec![];
        word_break_rec(s.as_str(), 0, &dict, &mut vec![], &mut res);
        res
    }
}

#[test]
fn test_word_break() {
    assert_eq!(
        HashSet::from(["cats and dog".to_string(), "cat sand dog".to_string()]), 
        Solution::word_break(
            "catsanddog".to_string(), 
            vec!["cat".to_string(),"cats".to_string(),"and".to_string(),"sand".to_string(),"dog".to_string()]
        ).into_iter().collect::<HashSet<String>>()
    );

    assert_eq!(
        HashSet::from(["pine apple pen apple".to_string(),"pineapple pen apple".to_string(),"pine applepen apple".to_string()]), 
        Solution::word_break(
            "pineapplepenapple".to_string(), 
            vec!["apple".to_string(), "pen".to_string(),"applepen".to_string(),"pine".to_string(),"pineapple".to_string()]
        ).into_iter().collect::<HashSet<String>>()
    );

    assert_eq!(
        Vec::<String>::new(), 
        Solution::word_break(
            "catsandog".to_string(), 
            vec!["cats".to_string(),"dog".to_string(),"sand".to_string(),"and".to_string(),"cat".to_string()]
        )
    );
}    
