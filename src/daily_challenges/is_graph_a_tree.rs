use std::collections::HashSet;

/// Check if a given graph is tree or not
///
/// You are given an undirected graph of N nodes (numbered from 0 to N-1) and M edges. Return 1 if the graph is a tree, else return 0.
///
/// Note: The input graph can have self-loops and multiple edges.
///
/// Example 1:
///
/// Input:
/// N = 4, M = 3
/// G = [[0, 1], [1, 2], [1, 3]]
/// Output:
/// 1
/// Explanation:
/// Every node is reachable and the graph has no loops, so it is a tree
///
/// Example 2:
///
/// Input:
/// N = 4, M = 3
/// G = [[0, 1], [1, 2], [2, 0]]
/// Output:
/// 0
/// Explanation:
/// 3 is not connected to any node and there is a loop 0->1->2->0, so it is not a tree.
///
/// Your Task:  
/// You don't need to read input or print anything. Your task is to complete the function isTree() which takes the integer N (the number nodes in the input graph) and the edges representing the graph as input parameters and returns 1 if the input graph is a tree, else 0.
///
/// Expected Time Complexity: O(N+M)
/// Expected Auxiliary Space: O(N)
///
/// Constraints:
/// 1 <= N <= 2*105
/// 0 <= M <= 2*105
///
pub fn is_graph_a_tree(n: &usize, edges: &Vec<(usize, usize)>) -> Result<bool, String> {
    // If graph is empty, its not a tree
    if *n == 0 {
        return Ok(false);
    }

    // Build an adjacency list
    let mut adjacency_list: Vec<HashSet<usize>> = vec![HashSet::new(); *n];
    for (from_node, to_node) in edges {
        adjacency_list
            .get_mut(*from_node)
            .ok_or("Edge refer to an invalid vertex")?
            .insert(*to_node);
        adjacency_list
            .get_mut(*to_node)
            .ok_or("Edge refer to an invalid vertex")?
            .insert(*from_node);
    }

    // Then traverse the graph depth first
    // Note: for each vertex push in the stack, we keep track of its
    // parent vertex (the vertex we came for), to be able to ignore it later
    let mut seen = HashSet::new();
    let mut stack = Vec::new();
    // push the first vertex to explore and its parent (which is not relevant for first iteration)
    stack.push((0, 0));

    while let Some((current_vertex, current_vertex_parent)) = stack.pop() {
        // If we find a seen vertex, there is a loop
        // so the graph is not a tree
        if seen.contains(&current_vertex) {
            return Ok(false);
        }

        // Mark the vertex as seen
        seen.insert(current_vertex);

        let current_vertex_neighbors = adjacency_list.get(current_vertex).unwrap();

        // Add the neighbor vertices to the queue
        for neighbor_vertex in current_vertex_neighbors {
            // If there is a self relation, the graph is not a tree
            if neighbor_vertex == &current_vertex {
                return Ok(false);
            }

            // ignore the edge used to come from previous vertex
            // to current vertex
            if *neighbor_vertex != current_vertex_parent {
                stack.push((*neighbor_vertex, current_vertex));
            }
        }
    }

    // If graph traversal found less vertices
    // than the total number of vertices in the graph,
    // there is an isolated part to the graph so it's
    // not a tree
    if seen.len() != *n {
        return Ok(false);
    }

    // Else it's a tree
    Ok(true)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_detect_valid_trees() {
        let graph_size = 4;
        let edges = vec![(0, 1), (1, 2), (1, 3)];

        assert_eq!(is_graph_a_tree(&graph_size, &edges), Ok(true));
    }

    #[test]
    fn should_detect_invalid_graph() {
        let graph_size = 2;
        let edges = vec![(0, 1), (1, 2)];

        assert_eq!(
            is_graph_a_tree(&graph_size, &edges),
            Err("Edge refer to an invalid vertex".to_string())
        );
    }

    #[test]
    fn should_detect_empty_graph() {
        let graph_size = 0;
        let edges = vec![];

        assert_eq!(is_graph_a_tree(&graph_size, &edges), Ok(false));
    }

    #[test]
    fn should_detect_graph_with_isolated_part() {
        let graph_size = 3;
        let edges = vec![(0, 1)];

        assert_eq!(is_graph_a_tree(&graph_size, &edges), Ok(false));
    }

    #[test]
    fn should_detect_graph_with_self_loop() {
        let graph_size = 2;
        let edges = vec![(0, 1), (1, 1)];

        assert_eq!(is_graph_a_tree(&graph_size, &edges), Ok(false));
    }

    #[test]
    fn should_detect_graph_with_loop() {
        let graph_size = 4;
        let edges = vec![(0, 1), (1, 2), (2, 3), (1, 3)];

        assert_eq!(is_graph_a_tree(&graph_size, &edges), Ok(false));
    }
}
