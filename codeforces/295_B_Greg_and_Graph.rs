use std::io::{stdin, Read,};
use std::fmt::Write;

fn read_input() -> (usize, Vec<Vec<u64>>, Vec<usize>) {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace().map(|word| word.parse().unwrap());
    let n = input.next().unwrap() as usize;
    let adj_matrix = (0..n).map(|_| (0..n).map(|_| input.next().unwrap()).collect()).collect();
    let node_order = (0..n).map(|_| input.next().unwrap()  as usize - 1).collect();
    (n, adj_matrix, node_order)
}

fn print_output(lengths: &[u64]) {
    let mut buffer = String::new();
    for &length in lengths {
        write!(&mut buffer, "{} ", length).unwrap();
    }
    println!("{}", buffer);
}

fn floyd_warshall(n: usize, adj_matrix: &mut [Vec<u64>], node_order: &[usize]) -> Vec<u64> {
    let mut visited = vec![false; n];
    let mut lengths = vec![];
    for &k in node_order.iter() {
        visited[k] = true;
        let mut total_length = 0;
        for v in 0..n {
            for u in 0..n {
                if adj_matrix[v][k] + adj_matrix[k][u]  < adj_matrix [v][u] { 
                    adj_matrix[v][u] = adj_matrix[v][k] + adj_matrix[k][u];
                }
                if visited[v] && visited[u] {
                    total_length += adj_matrix[v][u]
                }
            }
        }
        lengths.push(total_length);
    }
    lengths.reverse();
    lengths
}

fn main() {
    let (n, mut adj_matrix, mut node_order) = read_input();
    node_order.reverse();
    print_output(&floyd_warshall(n, &mut adj_matrix, &node_order));
}
