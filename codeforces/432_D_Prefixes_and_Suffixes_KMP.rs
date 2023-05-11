use std::io;
use std::fmt::Write as _;

fn kmp_function(string: &[u8]) -> Vec<usize> {
    let n = string.len();
    let mut kmp = vec![0; n];

    for i in 1..n {
        let mut j = i;
        while j > 0 && string[kmp[j - 1]] != string[i] {
            j = kmp[j - 1];
        }
        kmp[i] = if j > 0 { kmp[j - 1] + 1 } else { 0 }
    }
    kmp
}

fn prefixes_and_suffixes(string: &[u8]) -> Vec<(usize, u32)> {
    let n = string.len();
    let  kmp = kmp_function(string);
    let mut cnts = vec![0; n];
    let mut parents = vec![0; n];
    let mut res = vec![(n, 1)];

    for i in 1..n {
        if kmp[i] > 0 {
            cnts[kmp[i]] += 1;
            parents[kmp[i]] = kmp[kmp[i] - 1];
        }
    }
    for i in (1..n).rev() {
        cnts[parents[i]] += cnts[i];
    }
    let mut current = kmp[n - 1];
    while current != 0 {        
        res.push((current, cnts[current] + 1));
        current = parents[current];
    }    
    res
}

fn main() {        
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let res = prefixes_and_suffixes(buffer.trim().as_bytes());

    let mut out = String::new();

    writeln!(&mut out, "{}", res.len()).unwrap();
    for &(length, cnt) in res.iter().rev() {
        writeln!(out, "{} {}", length, cnt).unwrap();
    }
    print!("{}", out)    
}

#[test]
fn test_kmp_function() {
    assert_eq!([0, 1, 2, 3], kmp_function("aaaa".as_bytes()).as_slice());
    assert_eq!([0, 1, 2, 0], kmp_function("aaab".as_bytes()).as_slice());
    assert_eq!([0, 0, 1, 2, 3, 4], kmp_function("ababab".as_bytes()).as_slice());
}
