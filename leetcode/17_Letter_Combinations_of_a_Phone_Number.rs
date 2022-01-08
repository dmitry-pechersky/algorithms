struct Solution {
}

use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}


impl Solution {

    fn _rec(digits: &Vec<char>, n: usize, pos: usize, combination: &mut String, mapping: &HashMap<char, Vec<char>>, combinations: &mut Vec<String>) {
        for ch in &mapping[&digits[pos]] {
            combination.push(*ch);
            if pos + 1 < n {
                Self::_rec(digits, n, pos + 1, combination, mapping, combinations);
            } else {
                combinations.push(combination.clone());
            }
            combination.pop();
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits: Vec<char> = digits.chars().collect();
        let n = digits.len();
        let mapping = hashmap![
            '2' => vec!['a', 'b', 'c'], 
            '3' => vec!['d', 'e', 'f'], 
            '4' => vec!['g', 'h', 'i'], 
            '5' => vec!['j', 'k', 'l'], 
            '6' => vec!['m', 'n', 'o'], 
            '7' => vec!['p', 'q', 'r', 's'], 
            '8' => vec!['t', 'u', 'v'], 
            '9' => vec!['w', 'x', 'y', 'z']];
        let mut combinations = Vec::<String>::new();
        let mut combination = String::new();
        if n > 0 {
            Self::_rec(&digits, n, 0, &mut combination, &mapping, &mut combinations);
        }
        combinations
    }
}

#[cfg(test)]
mod letter_combinations_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::letter_combinations(String::from("")), Vec::<String>::new());
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::letter_combinations(String::from("2")), vec![String::from("a"), String::from("b"), String::from("c")]);
    }
}

