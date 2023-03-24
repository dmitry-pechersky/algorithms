use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
        val,
        left: None,
        right: None
        }
    }
}

#[derive(Clone, Copy)]
enum Token{
    Open,
    Close,
    None,
    Digit(u32),
}

impl Token {
    fn new(ch: char) -> Self {
        match ch {
            '(' => Token::Open ,
            ')' => Token::Close,
            'N' => Token::None ,
            ch if ch.is_digit(10) => Token::Digit(ch.to_digit(10).unwrap()),
            _ => unreachable!()
        }        
    }
}

struct Codec {}

impl Codec {

    const SMALLEST_VALUE: i32 = -1000;

    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn se_node(node: &Option<Rc<RefCell<TreeNode>>>, str: &mut String) {
            if let Some(node) = node.as_ref() {
                let node = node.borrow();
                str.push('(');
                str.push_str(&(node.val - Codec::SMALLEST_VALUE).to_string());
                if node.left.is_some() || node.right.is_some() {
                    se_node(&node.left, str);
                }
                if node.right.is_some() {
                    se_node(&node.right, str);    
                }
                str.push(')')
            } else {
                str.push('N')
            }
        }
        let mut str = String::new();
        if root.is_some() {
            se_node(&root, &mut str);
        }
        str
    }
    
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn de_value(tokens: &Vec<Token>, i: &mut usize) -> i32 {
            let mut val = 0;
            loop {
                if let Token::Digit(digit) = tokens[*i] {
                    val = val * 10 + digit;
                    *i += 1;
                } else {
                    break val as i32 + Codec::SMALLEST_VALUE;
                }
            }
        }

        fn de_child(tokens: &Vec<Token>, i: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            match tokens[*i] {
                Token::Open => {
                    de_node(tokens, i)
                },
                Token::None => {
                    *i += 1;
                    None
                },
                Token::Close => None,
                _ => unreachable!(),
            }
        }

        fn de_node(tokens: &Vec<Token>, i: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            *i += 1;
            let val = de_value(tokens, i);
            let left = de_child(tokens, i);
            let right = de_child(tokens, i);
            *i += 1;
            Some(Rc::new(RefCell::new(TreeNode {val, left, right})))
        }

        let tokens = data.chars().into_iter().map(|ch| Token::new(ch)).collect::<Vec<_>>();
        if tokens.is_empty() {
            None
        } else {
            de_node(&tokens, &mut 0)
        }
    }
}

#[test]
fn test_1() {
    let str = "(1(2)(3(4)(5)))";
    let codec = Codec::new();
    assert_eq!(str, codec.serialize(codec.deserialize(str.to_string())));
}

#[test]
fn test_2() {
    let str = "";
    let codec = Codec::new();
    assert_eq!(str, codec.serialize(codec.deserialize(str.to_string())));
}
