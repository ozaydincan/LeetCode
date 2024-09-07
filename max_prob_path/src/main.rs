struct Solution;

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let n = n as usize;
        let start_node = start_node as usize;
        let end_node = end_node as usize;
        
        let mut graph: HashMap<usize, Vec<(usize, f64)>> = HashMap::new();
        
        for (i, edge) in edges.iter().enumerate() {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            let prob = succ_prob[i];
            
            graph.entry(a).or_insert_with(Vec::new).push((b, prob));
            graph.entry(b).or_insert_with(Vec::new).push((a, prob));
        }

        #[derive(Debug, PartialEq, Eq)]
        struct State {
            prob: f64,
            node: usize,
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                // Reverse the ordering to create a max-heap
                other.prob.partial_cmp(&self.prob).unwrap_or(Ordering::Equal)
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                // Reverse the ordering to create a max-heap
                other.prob.partial_cmp(&self.prob)
            }
        }

        let mut max_heap = BinaryHeap::new();
        max_heap.push(State { prob: 1.0, node: start_node });

        let mut best_probs = vec![0.0; n];
        best_probs[start_node] = 1.0;

        while let Some(State { prob: current_prob, node }) = max_heap.pop() {
            if node == end_node {
                return current_prob;
            }

            if current_prob < best_probs[node] {
                continue;
            }

            if let Some(neighbors) = graph.get(&node) {
                for &(neighbor, edge_prob) in neighbors {
                    let new_prob = current_prob * edge_prob;
                    if new_prob > best_probs[neighbor] {
                        best_probs[neighbor] = new_prob;
                        max_heap.push(State { prob: new_prob, node: neighbor });
                    }
                }
            }
        }
        
        0.0
    }
}



fn main() {
    println!("Hello, world!");
}


#[cfg(test)]

mod test{
    use super::*;

    #[test]

    fn testing_1(){
        let n:i32 = 3;
        let edges: Vec<Vec<i32>> = vec![vec![0,1],vec![1,2], vec![0,2]];
        let succ_prob:Vec<f64> = vec![0.5,0.5,0.2];
        assert_eq!(Solution::max_probability(n, edges, succ_prob, 0, 2), 0.25000)
    }
}
