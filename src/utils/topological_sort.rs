use std::collections::{HashMap, HashSet, VecDeque};

pub fn topological_sort(graph: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut visited: HashSet<i32> = HashSet::new();
    let mut stack: VecDeque<i32> = VecDeque::new();

    // Добавляем в стек все вершины, которые не имеют зависимостей
    for node in graph.keys() {
        if graph[node].is_empty() {
            stack.push_back(*node);
        }
    }

    // Обходим граф
    while let Some(node) = stack.pop_front() {
        if !visited.contains(&node) {
            visited.insert(node);
            result.push(node);

            // Добавляем в стек все вершины, которые могут быть обработаны после текущей
            for (next_node, structure) in graph.iter() {
                if !visited.contains(next_node) && structure.contains(&node) {
                    let all_dependencies_visited =
                        structure.iter().all(|dep| visited.contains(dep));
                    if all_dependencies_visited {
                        stack.push_back(*next_node);
                    }
                }
            }
        }
    }

    result
}
