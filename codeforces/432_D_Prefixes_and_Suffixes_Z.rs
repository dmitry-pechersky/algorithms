use std::io;
use std::fmt::Write as _;

fn z_function(string: &[u8]) -> Vec<usize> {
    let n = string.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i <= r {
            z[i] = z[i - l].min(r - i + 1);
        }
        while i + z[i] < n && string[i + z[i]] == string[z[i]] {
            z[i] += 1;
        }
        if z[i] > 0 && i + z[i] - 1 > r { 
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}

fn prefixes_and_suffixes(string: &[u8]) -> Vec<(usize, u32)> {
    let n = string.len();
    let z = z_function(string);
    let mut cnts = vec![0; n];
    let mut res = vec![];
    for i in 1..n {
        cnts[z[i]] += 1;
    }
    for i in  (1..n - 1).rev() {
        cnts[i] += cnts[i + 1];
    }
    for i in (1..n).rev() {
        if i + z[i] == n {
            res.push((z[i], cnts[z[i]] + 1))
        }
    }
    res.push((n, 1));
    res
}

fn main() {    
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let res = prefixes_and_suffixes(buffer.trim().as_bytes());

    let mut out = String::new();

    writeln!(&mut out, "{}", res.len()).unwrap();
    for (length, cnt) in res {
        writeln!(out, "{} {}", length, cnt).unwrap();
    }
    print!("{}", out)
}

#[test]
fn test_z_function() {
    assert_eq!(vec![0], z_function("a".as_bytes()));
    assert_eq!(vec![0, 3, 2, 1], z_function("aaaa".as_bytes()));
    assert_eq!(vec![0, 0, 4, 0, 2, 0], z_function("ababab".as_bytes()));
}
