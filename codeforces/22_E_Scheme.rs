use std::{io::stdin, fmt::Write};

fn read_input() -> Vec<usize> {
    let mut lines = stdin().lines();
    let n: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut edges = vec![0; n as usize];
    for (v, u) in lines.next().unwrap().unwrap().trim().split_whitespace().map(|value| value.parse::<usize>().unwrap() - 1).enumerate() {
        edges[v] = u;
    }
    edges
}

fn write_output(edges: &Vec<(usize, usize)>) {
    let mut buffer = String::new();
    writeln!(&mut buffer, "{}", edges.len()).unwrap();
    for &(v, u) in edges {
        writeln!(&mut buffer, "{} {}", v + 1, u + 1).unwrap();
    }
    print!("{}", buffer);
}

fn main() {
    let edges = read_input();
    let n = edges.len();

    let mut stack = vec![];
    let mut visited = vec![false; n];
    let mut in_stack = vec![false; n];
    let mut vert_to_comp = vec![0; n];
    let mut comp_cycle_vert = vec![];
    for mut v in 0..n {
        while ! visited[v] { 
            visited[v] = true;
            in_stack[v] = true;
            stack.push(v);
            v = edges[v];
        }
        if in_stack[v] {
            vert_to_comp[v] = comp_cycle_vert.len();
            comp_cycle_vert.push(v);
        }
        let component = vert_to_comp[v]; 

        while let Some(v) = stack.pop() {
            vert_to_comp[v] = component;
            in_stack[v] = false;
        }
    }

    let comp_n = comp_cycle_vert.len();
    let mut vert_has_in_edges = vec![false; n];
    for v in 0..n {
        vert_has_in_edges[edges[v]] = true;
    }

    let mut new_edges = vec![];
    let mut used_comp = vec![false; comp_n];
    for (v, _) in vert_has_in_edges.into_iter().enumerate().filter(|(_, has_edges)| !has_edges) {
        let u = comp_cycle_vert[(vert_to_comp[v] + 1) % comp_n];
        new_edges.push((u, v));
        used_comp[vert_to_comp[v]] = true;
    }
    if comp_n > 1 {
        for (comp, _) in used_comp.into_iter().enumerate().filter(|(_, used)| !used) {
            let v = comp_cycle_vert[comp];
            let u = comp_cycle_vert[(comp + 1) % comp_n];
            new_edges.push((u, v));
        }    
    }

    write_output(&new_edges);
}
