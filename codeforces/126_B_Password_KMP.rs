use std::io;

fn kmp_function(string: &[u8]) -> Vec<usize> {
    let n = string.len();
    let mut kmp = vec![0; n];
    for i in 1..n {
        let mut j = i;
        while j > 0 && string[kmp[j - 1]] != string[i] {
            j = kmp[j - 1];
        }
        if j > 0 {
            kmp[i] = kmp[j - 1] + 1;
        }
    }
    kmp
}

fn password_length(string: &str) -> usize {
    let string = string.as_bytes();
    let n = string.len();
    if n < 2 {
        return 0;
    }
    let kmp= kmp_function(string);
    let max_length = *kmp[0..n - 1].iter().max().unwrap();
    let mut i = n - 1;
    while kmp[i] > max_length {
        i = kmp[i] - 1;
    }
    kmp[i]
}

fn main() { 
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let string = buffer.trim();
    let length = password_length(string);
    println!("{}", if length > 0 { &string[0..length] } else { "Just a legend" })
}

#[test]
fn test_kmp_function() {
    assert_eq!([0, 1, 2, 3], kmp_function("aaaa".as_bytes()).as_slice());
    assert_eq!([0, 1, 2, 3], kmp_function("aaaa".as_bytes()).as_slice());
    assert_eq!([0, 0, 1, 2, 3, 0], kmp_function("ababac".as_bytes()).as_slice());
}

#[test]
fn test_password_length() {
    assert_eq!(3, password_length("fixprefixsuffix"));
    assert_eq!(0, password_length("abcdabc"));
    assert_eq!(0, password_length("abcabcw"));
    assert_eq!(2, password_length("qwqwqw"));
    assert_eq!(2, password_length("abcabcab"));
    assert_eq!(0, password_length("a"));
}
