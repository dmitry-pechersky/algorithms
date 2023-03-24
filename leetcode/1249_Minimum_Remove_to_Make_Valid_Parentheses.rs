struct Solution {}

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let s =  s.as_bytes();
        let mut include = vec![true; s.len()];
        let mut stack = vec![];
        for (i,ch) in s.iter().enumerate() {
            if *ch == b')' {
                if stack.pop().is_none() {
                    include[i] = false;
                }
            } else if *ch == b'(' {
                stack.push(i);
            }
        }
        while let Some(i) = stack.pop() {
            include[i] = false;
        }
        s.iter().enumerate().filter_map(|(i, ch)| if include[i] { Some(*ch as char) } else { None }).collect()
    }
}
