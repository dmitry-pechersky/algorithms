use std::io;

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

fn password_length(string: &str) -> usize {
    let n = string.len();
    let z = z_function(string.as_bytes());
    let max_length = (1..n).map(|i| if i + z[i] < n { z[i] } else { z[i] - 1 }).max().unwrap_or(0);
    for i in n - max_length..n {
        if i + z[i] == n {
            return z[i];
        }
    }
    0
}

fn main() { 
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let string = buffer.trim();
    let length = password_length(string);
    println!("{}", if length > 0 { &string[0..length] } else { "Just a legend" } );
}

#[test]
fn test_z_function() {
    assert_eq!([0], z_function("a".as_bytes()).as_slice());
    assert_eq!([0, 3, 2, 1], z_function("aaaa".as_bytes()).as_slice());
    assert_eq!([0, 0, 4, 0, 2, 0], z_function("qwqwqw".as_bytes()).as_slice())
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
