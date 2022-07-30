use std::collections::{HashMap, VecDeque, HashSet};


struct Solution {}

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let (source, target) = (source as usize, target as usize);
        let bus_n = routes.len();        
        let mut stop_bus_dic = HashMap::new();
        let mut adj_list = vec![Vec::new(); bus_n];

        for bus in 0..bus_n {
            for stop in routes[bus].iter().map(|x| *x as usize) {
                stop_bus_dic.entry(stop).or_insert(vec![]).push(bus);
            }
        }
        for buses in stop_bus_dic.values() {
            for i in 0..buses.len() {
                for j in i+1..buses.len() {
                    adj_list[buses[i]].push(buses[j]);
                    adj_list[buses[j]].push(buses[i]);
                }
            }
        }
        if let (Some(target_buses), Some(source_buses)) = (stop_bus_dic.get(&target), stop_bus_dic.get(&source)) {
            let targets = target_buses.iter().copied().collect::<HashSet<_>>();
            let mut queue = source_buses.iter().map(|bus| (*bus, 1)).collect::<VecDeque<_>>();
            let mut expended = vec![false; bus_n];
            while let Some((v, cost)) = queue.pop_front() {
                if targets.contains(&v) {
                    return cost;
                }
                if ! expended[v] {
                    expended[v] = true;
                    for u in &adj_list[v] {
                        if ! expended[*u] {
                            queue.push_back((*u, cost + 1));
                        }
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test_1() {
    assert_eq!(2, Solution::num_buses_to_destination(vec![vec![1,2,7], vec![3,6,7]], 1, 6));
}

#[test]
fn test_2() {
    assert_eq!(-1, Solution::num_buses_to_destination(vec![vec![7,12], vec![4,5,15], vec![6], vec![15,19], vec![9,12,13]], 15, 12));
}
