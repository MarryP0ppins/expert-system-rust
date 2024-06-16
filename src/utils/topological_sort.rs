use std::collections::{HashMap, HashSet, VecDeque};

pub fn topological_sort(graph: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut in_degree = HashMap::new();
    let mut sorted = Vec::with_capacity(graph.len());
    let mut queue = VecDeque::new();

    for (&node, neighbors) in graph.iter() {
        in_degree.entry(node).or_insert(0);
        for &neighbor in neighbors {
            *in_degree.entry(neighbor).or_insert(0) += 1;
        }
    }

    for (&node, &degree) in in_degree.iter() {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted.reverse();
    if sorted.len() == graph.len() {
        sorted
    } else {
        Vec::new()
    }
}
