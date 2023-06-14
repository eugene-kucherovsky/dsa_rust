
// Has Path ?
// Difficulty Level : basic


// Time complexity: O(n + e)
// Space Complexity: O(n)

// n - the number of vertices
// e - the number of edges
use std::collections::HashMap;

pub fn has_path(graph: &HashMap<String, Vec<String>>, src: String, dst: String) -> bool {
    let mut queue = vec![src];

    while queue.len() > 0 {
        let current = queue.remove(0);

        if current == dst {
            return true;
        }

        if graph.contains_key(&current) {
            for neighbor in graph[&current].iter() {
                queue.push(neighbor.to_string());
            }
        }
    }
    return false;
}

// recursive version
pub fn rec(graph: &HashMap<String, Vec<String>>, src: String, dst: String) -> bool {
    if src == dst {
        return true;
    }

    if graph.contains_key(&src) {
        for neighbor in graph[&src].iter() {

            // dst.clone() - does not work without clone() !
            if rec(&graph, neighbor.to_string(), dst.clone()) == true {
                return true;
            }
        }
    }
    return false;
}