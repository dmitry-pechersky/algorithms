use std::{io, fmt::Debug, str::FromStr};


fn read_number<T>() -> T 
where T: FromStr, <T as FromStr>::Err: Debug {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}

fn read_numbers<T>() -> Vec<T>
where T: FromStr, <T as FromStr>::Err: Debug {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().split_whitespace().map(|word| word.parse().unwrap()).collect()
}

struct LCA {
    ett: Vec<usize>,
    firsts: Vec<usize>,
    rmq: RMQ
}

impl LCA {
    fn new(root: usize, adj_list: &Vec<Vec<usize>>) -> Self {
        fn dfs(v: usize, adj_list: &Vec<Vec<usize>>, depth: u16, ett: &mut Vec<usize>, depths: &mut Vec<u16>) {
            ett.push(v);
            depths.push(depth);
            for &u in &adj_list[v] {
                dfs(u, adj_list, depth + 1, ett, depths);
                ett.push(v);
                depths.push(depth);
            }
        }

        let n = adj_list.len();
        let mut depths = vec![];
        let mut ett = vec![];
        dfs(root, adj_list, 0, &mut ett, &mut depths);

        let mut firsts = vec![usize::MAX; n];
        for (i, &v) in ett.iter().enumerate() {
            if firsts[v] == usize::MAX {
                firsts[v] = i;
            }
        }
        LCA { ett, firsts, rmq: RMQ::new(depths) }
    }

    fn query(&self, v: usize, u: usize) -> usize {
        let (first_v, first_u) = (self.firsts[v], self.firsts[u]);
        self.ett[self.rmq.query(first_v.min(first_u), first_v.max(first_u))]
    }
}

struct RMQ {
    dp: Vec<Vec<usize>>,
    nums: Vec<u16>,
}

impl RMQ {
    const fn ilog2(n: usize) -> u32 {
        usize::BITS - n.leading_zeros() - 1
    }

    fn new(nums: Vec<u16>) -> Self {
        let n = nums.len();
        let k = Self::ilog2(n) as usize;
        let mut dp = vec![vec![0; n]; k as usize + 1];
        for i in 0..n {
            dp[0][i] = i;
        }
        for i in 1..=k {
            let pow_2_i_minus_1 = 1 << (i - 1);
            for j in 0..n - pow_2_i_minus_1 {
                let (dp_1, dp_2) = (dp[i - 1][j], dp[i - 1][j + pow_2_i_minus_1]);
                dp[i][j] = if nums[dp_1] < nums[dp_2] { dp_1 } else { dp_2 };
            }
        }
        RMQ { dp, nums }
    }

    fn query(&self, l: usize, r: usize) -> usize {
        if l == r {
            l
        } else {
            let k = Self::ilog2(r - l) as usize;
            let pow_2_k = 1 << k;
            let (dp_1, dp_2) = (self.dp[k][l], self.dp[k][r - pow_2_k + 1]);
            if self.nums[dp_1] < self.nums[dp_2] { dp_1 } else { dp_2 }    
        }
    }
}

fn tree_root(adj_list: &Vec<Vec<usize>>) -> usize {
    let n = adj_list.len();
    let mut is_childs = vec![false; n];
    for v in 0..n {
        for &u in &adj_list[v] {
            is_childs[u] = true;
        }
    }
    for (i, is_child) in is_childs.into_iter().enumerate() {
        if ! is_child {
            return i;
        }
    }
    unreachable!()
}

fn main() {
    for t in 0..read_number() {
        println!("Case {}:", t + 1);

        let n = read_number();

        let adj_list: Vec<Vec<usize>> = (0..n).map(|_| read_numbers().into_iter().skip(1).map(|u: usize| u - 1).collect()).collect();

        let lca = LCA::new(tree_root(&adj_list), &adj_list);

        for _ in 0..read_number() {
            let query = read_numbers::<usize>();
            let (v, u) = (query[0] - 1, query[1] - 1);
            println!("{}", lca.query(v, u) + 1);
        }
    }
}
