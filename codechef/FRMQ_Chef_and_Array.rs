use std::{io, str::FromStr, fmt::Debug};

struct RMQ { dp: Vec<Vec<u32>> }

fn ilog2(n: usize) -> u32 {
    usize::BITS - n.leading_zeros() - 1
}

impl RMQ {
    fn new(nums: &[u32]) -> Self {
        let n = nums.len(); 
        let k = ilog2(n) as usize;
        let mut dp = vec![Vec::from(nums)];
        for j in 1..k + 1 {
            let (pow_2_j, pow_2_j_minus_1) = (1 << j, 1 << (j - 1));
            dp.push(vec![0; n - pow_2_j + 1]);
            for i in 0..n - pow_2_j + 1 {
                dp[j][i] = dp[j - 1][i].max(dp[j - 1][i + pow_2_j_minus_1]);
            }
        }
        Self { dp }
    }

    fn query(&self, l: usize, r: usize) -> u32 {
        let k = ilog2(r - l + 1);
        let pow2_k = 1 << k;
        self.dp[k as usize][l].max(self.dp[k as usize][r + 1 - pow2_k])
    }     
}

fn read_numbers<T>() -> Vec<T>
where T: FromStr, <T as FromStr>::Err: Debug
{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().split_whitespace().map(|word| word.parse().unwrap()).collect()
}

fn main() {
    let n = read_numbers::<usize>()[0];
    let nums = read_numbers::<u32>();
    let v = read_numbers::<usize>();
    let (m, mut x, mut y) = (v[0], v[1], v[2]);
    let mut res = 0;

    let rmq = RMQ::new(&nums);
    for _ in 0..m {
        res += rmq.query(x.min(y), x.max(y)) as u64;
        x = (x + 7) % (n  - 1);
        y = (y + 11) % n;
    }    
    println!("{}", res);
}

#[test]
fn test_rmq() {
    assert_eq!(
        RMQ::new(&[1]).dp,
        vec![vec![1]]
    );
    assert_eq!(
        RMQ::new(&[3, 1, 2, 0, 1]).dp, 
        vec![vec![3, 1, 2, 0, 1], vec![3, 2, 2, 1], vec![3, 2]]
    );

    let rmq = RMQ::new(&[3, 1, 2, 0, 1]);
    assert!(
        vec![3, 2, 2].into_iter().eq(
            [(0, 4), (1, 4), (2, 2)].iter().map(|&(l, r)| rmq.query(l, r))
        )
    );
}
