use std::collections::{HashMap, HashSet, VecDeque};

use super::identifiable::Identifiable;

/// A basic oriented graph implementation
///
/// It is 'naive' in the sens that it plays around references and lifetime rust safety
/// by using String id as pointers for the vertices hashmap keys, and cloned string
/// to refer the vertices in the edges.
/// It skips all the safety rust can bring, forces to do runtime check and to be really careful
/// when cleaning vertices/edges in order to keep the graph correct.
///
/// To benefit of rust safety, more advanced memory management methods should be used
/// (see [here](https://stackoverflow.com/questions/34747464/implement-graph-like-data-structure-in-rust)
/// and [here](https://github.com/nrc/r4cppp/blob/master/graphs/README.md) for more infos)
#[derive(Debug)]
pub struct OrientedGraph<T: Identifiable> {
    /// List of the graph vertices
    pub vertices: HashMap<String, T>,

    /// List of the graph edges
    ///
    /// Note: The way it is structured allow to express oriented edges
    /// Example: For vertices A and B, you can express:
    /// - A -> B by storing the entry { A: [B] }
    /// - B -> A by storing the entry { B: [A] }
    pub edges: HashMap<String, HashSet<String>>,
}

impl<T: Identifiable> OrientedGraph<T> {
    /// Instantiate a new oriented graph
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Add a vertex into the graph, and Err if it already exists
    pub fn add_vertex(&mut self, vertex: T) -> Result<(), String> {
        let vertex_id = vertex.get_id();

        // Err if vertex already exists
        // Or insert the vertex in the graph
        if self.vertex_exists(&vertex_id) {
            Err("Vertex ".to_owned() + &vertex_id + " already exists")
        } else {
            self.vertices.insert(vertex_id.clone(), vertex);
            self.edges.insert(vertex_id, HashSet::new());
            Ok(())
        }
    }

    /// Add an edge into the graph, and Err if it already exists, of if it targets invalid vertices
    pub fn add_edge(&mut self, vertex1_id: String, vertex2_id: String) -> Result<(), String> {
        // Check that vertices exist, or err
        self.vertex_exists_or_err(&vertex1_id)?;
        self.vertex_exists_or_err(&vertex2_id)?;

        // Insert the new edge
        let result = self
            .edges
            .get_mut(&vertex1_id)
            .unwrap()
            .insert(vertex2_id.clone());

        match result {
            true => Ok(()),
            false => Err("Edge between from vertex ".to_owned()
                + &vertex1_id
                + " to vertex "
                + &vertex2_id
                + " already exists"),
        }
    }

    /// Try to remove an edge and return an Err if it was not present
    pub fn remove_edge(&mut self, vertex1_id: String, vertex2_id: String) -> Result<(), String> {
        // Check that vertices exist, or err
        self.vertex_exists_or_err(&vertex1_id)?;
        self.vertex_exists_or_err(&vertex2_id)?;

        // Try to remove the value
        let result = self
            .edges
            .get_mut(&vertex1_id)
            .map(|ids| ids.remove(&vertex2_id));

        // If the edge was present and removed, return Ok, else Err
        if result == Some(true) {
            Ok(())
        } else {
            Err("Edge from vertex ".to_owned()
                + &vertex1_id
                + " to vertex "
                + &vertex2_id
                + " does not exist")
        }
    }

    /// Try to remove a vertex and return an Err if it was not present
    pub fn remove_vertex(&mut self, vertex_id: String) -> Result<(), String> {
        // Check that vertex exist, or err
        self.vertex_exists_or_err(&vertex_id)?;

        // Remove edges from this vertex
        self.edges.remove(&vertex_id);

        // Remove edges to this vertex
        for edges_set in self.edges.values_mut() {
            edges_set.retain(|elem| *elem != vertex_id);
        }

        // Remove the vertex
        self.vertices.remove(&vertex_id);

        Ok(())
    }

    /// Denote if a vertex exists
    pub fn vertex_exists(&self, id: &String) -> bool {
        self.vertices.contains_key(id)
    }

    /// Check if a vertex exists and return Ok, or Err it does not exists
    pub fn vertex_exists_or_err(&self, id: &String) -> Result<(), String> {
        if self.vertex_exists(id) {
            Ok(())
        } else {
            Err("Vertex ".to_owned() + id + " does not exist")
        }
    }

    /// Traverse the graph depth fist starting from the vertex specified in parameter
    ///
    /// Note: this version is implemented using the call stack (recursive)
    pub fn traverse_depth_first_recursive(
        &self,
        start_vertex_id: String,
    ) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let mut seen = HashSet::new();

        self.internal_traverse_depth_first_recursive(start_vertex_id, &mut result, &mut seen)?;

        Ok(result)
    }

    /// Traverse the graph depth fist starting from the vertex specified in parameter
    ///
    /// Note: this version is implemented using an explicit stack and a loop (iterative)
    fn internal_traverse_depth_first_recursive(
        &self,
        vertex_id: String,
        result: &mut Vec<String>,
        seen: &mut HashSet<String>,
    ) -> Result<(), String> {
        // Check for invalid vertex
        self.vertex_exists_or_err(&vertex_id)?;

        // Add vertex to results
        result.push(vertex_id.clone());

        // Note vertex as visited
        seen.insert(vertex_id.clone());

        // Recursively traverse neighbors
        for neighbor_id in self.edges.get(&vertex_id).unwrap() {
            // if neighbor has not been visited yet
            if !seen.contains(neighbor_id) {
                self.internal_traverse_depth_first_recursive(neighbor_id.clone(), result, seen)?;
            }
        }

        Ok(())
    }

    /// Traverse the graph breadth fist starting from the vertex specified in parameter
    ///
    /// Note: this version is implemented using an explicit queue and a loop (iterative)
    pub fn traverse_depth_first_iterative(
        &self,
        start_vertex_id: String,
    ) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let mut seen = HashSet::new();
        let mut stack = Vec::new();

        // Init the stack with the starting node
        stack.push(start_vertex_id);

        while let Some(vertex_id) = stack.pop() {
            // Check that vertex is valid
            self.vertex_exists_or_err(&vertex_id)?;

            // Add vertex to result
            result.push(vertex_id.clone());

            // Mark vertex as seen
            seen.insert(vertex_id.clone());

            // Push neighbors vertices to the stack
            for neighbor_id in self.edges.get(&vertex_id).unwrap() {
                if !seen.contains(neighbor_id) {
                    stack.push(neighbor_id.clone());
                }
            }
        }

        Ok(result)
    }

    pub fn traverse_breadth_first(&self, start_vertex_id: String) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        // Init the queue with the starting node
        queue.push_back(start_vertex_id.clone());

        while let Some(vertex_id) = queue.pop_front() {
            // Check that vertex is valid
            self.vertex_exists_or_err(&vertex_id)?;

            // Mark vertex as seen
            seen.insert(vertex_id.clone());

            // Add vertex to result
            result.push(vertex_id.clone());

            // Push vertex unseen neighbors to the queue
            for neighbor_id in self.edges.get(&vertex_id).unwrap() {
                if !seen.contains(neighbor_id) {
                    queue.push_back(neighbor_id.clone());
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Basic struct for testing purpose
    #[derive(PartialEq, Clone, Debug)]
    struct City {
        pub name: String,
    }

    impl Identifiable for City {
        fn get_id(&self) -> String {
            self.name.clone()
        }
    }

    #[test]
    fn test_new() {
        let graph: OrientedGraph<City> = OrientedGraph::new();

        assert_eq!(graph.vertices, HashMap::new());
        assert_eq!(graph.edges, HashMap::new());
    }

    #[test]
    fn test_add_vertex() -> Result<(), String> {
        let mut graph: OrientedGraph<City> = OrientedGraph::new();

        let tokio = City {
            name: "Tokio".to_string(),
        };

        // Adding a city should work
        let mut result = graph.add_vertex(tokio.clone());
        assert_eq!(result, Ok(()));
        assert_eq!(graph.vertices.get("Tokio"), Some(&tokio));

        // Adding the same city should Err
        result = graph.add_vertex(tokio);
        assert_eq!(result, Err("Vertex Tokio already exists".to_string()));

        Ok(())
    }

    #[test]
    fn test_add_edge() -> Result<(), String> {
        let mut graph: OrientedGraph<City> = OrientedGraph::new();

        // Add cities
        let tokio = City {
            name: "Tokio".to_string(),
        };
        let vancouver = City {
            name: "Vancouver".to_string(),
        };
        let _ = graph.add_vertex(tokio.clone())?;
        let _ = graph.add_vertex(vancouver.clone())?;

        // Adding the edge should work
        let mut result = graph.add_edge(tokio.name.clone(), vancouver.name.clone());
        assert_eq!(result, Ok(()));
        assert_eq!(
            graph
                .edges
                .get(&tokio.name)
                .unwrap()
                .contains(&vancouver.name),
            true
        );

        // Adding an edge with an absent city should Err
        result = graph.add_edge(tokio.name.clone(), "Montreal".to_string());
        assert_eq!(result, Err("Vertex Montreal does not exist".to_string()));

        // Adding an existing edge should Err
        result = graph.add_edge(tokio.name.clone(), vancouver.name.clone());
        assert_eq!(
            result,
            Err("Edge between from vertex Tokio to vertex Vancouver already exists".to_string())
        );

        Ok(())
    }

    #[test]
    fn test_remove_edge() -> Result<(), String> {
        let mut graph: OrientedGraph<City> = OrientedGraph::new();

        // Add cities
        let tokio = City {
            name: "Tokio".to_string(),
        };
        let vancouver = City {
            name: "Vancouver".to_string(),
        };
        let _ = graph.add_vertex(tokio.clone())?;
        let _ = graph.add_vertex(vancouver.clone())?;
        let _ = graph.add_edge(tokio.name.clone(), vancouver.name.clone())?;

        // Should return error if vertex does not exists
        assert_eq!(
            graph.remove_edge(tokio.name.clone(), "Montreal".to_string()),
            Err("Vertex Montreal does not exist".to_string())
        );

        // Should return error if edge does not exists
        assert_eq!(
            graph.remove_edge(vancouver.name.clone(), tokio.name.clone()),
            Err("Edge from vertex Vancouver to vertex Tokio does not exist".to_string())
        );

        // Should remove edge
        assert_eq!(
            graph.remove_edge(tokio.name.clone(), vancouver.name.clone()),
            Ok(())
        );
        assert!(graph.edges.get(&tokio.name).unwrap().is_empty());

        Ok(())
    }

    #[test]
    fn test_remove_vertex() -> Result<(), String> {
        let mut graph: OrientedGraph<City> = OrientedGraph::new();

        // Add cities
        let tokio = City {
            name: "Tokio".to_string(),
        };
        let vancouver = City {
            name: "Vancouver".to_string(),
        };
        let _ = graph.add_vertex(tokio.clone())?;
        let _ = graph.add_vertex(vancouver.clone())?;

        // Add edges
        let _ = graph.add_edge(tokio.name.clone(), vancouver.name.clone())?;
        let _ = graph.add_edge(vancouver.name.clone(), tokio.name.clone())?;

        // Should return error if vertex does not exists
        assert_eq!(
            graph.remove_vertex("Montreal".to_string()),
            Err("Vertex Montreal does not exist".to_string())
        );

        // Should remove vertex and associated edges
        assert_eq!(graph.remove_vertex(tokio.name.clone()), Ok(()));
        assert_eq!(graph.vertices.get(&tokio.name), None);
        assert_eq!(graph.edges.get(&tokio.name), None);
        assert!(graph.edges.get(&vancouver.name).unwrap().is_empty());

        Ok(())
    }

    // Note: this test is very basic and should be improved
    #[test]
    fn test_traverse_depth_first_recursive() -> Result<(), String> {
        let mut graph: OrientedGraph<City> = OrientedGraph::new();

        // Add cities
        let tokio = City {
            name: "Tokio".to_string(),
        };
        let paris = City {
            name: "Paris".to_string(),
        };
        let vancouver = City {
            name: "Vancouver".to_string(),
        };
        let _ = graph.add_vertex(tokio.clone())?;
        let _ = graph.add_vertex(vancouver.clone())?;
        let _ = graph.add_vertex(paris.clone())?;

        // Add edges
        let _ = graph.add_edge(tokio.name.clone(), vancouver.name.clone())?;
        let _ = graph.add_edge(tokio.name.clone(), paris.name.clone())?;
        let _ = graph.add_edge(vancouver.name.clone(), tokio.name.clone())?;

        // Should return error if node is node in the graph
        assert_eq!(
            graph.traverse_depth_first_recursive("Montreal".to_string()),
            Err("Vertex Montreal does not exist".to_string())
        );

        // Should return the list of explored nodes
        let result = graph.traverse_depth_first_recursive(tokio.name.clone());

        print!("Result {:#?}", result);

        // This can not be tested easily as ordered is not guaranteed due to the
        // use of HashSet to store edges
        // assert_eq!(result, Ok(vec![tokio.name, vancouver.name, paris.name]));

        Ok(())
    }

    // Note: this test is very basic and should be improved
    #[test]
    fn test_traverse_depth_first_iterative() -> Result<(), String> {
        let mut graph: OrientedGraph<City> = OrientedGraph::new();

        // Add cities
        let tokio = City {
            name: "Tokio".to_string(),
        };
        let paris = City {
            name: "Paris".to_string(),
        };
        let vancouver = City {
            name: "Vancouver".to_string(),
        };
        let _ = graph.add_vertex(tokio.clone())?;
        let _ = graph.add_vertex(vancouver.clone())?;
        let _ = graph.add_vertex(paris.clone())?;

        // Add edges
        let _ = graph.add_edge(tokio.name.clone(), vancouver.name.clone())?;
        let _ = graph.add_edge(tokio.name.clone(), paris.name.clone())?;
        let _ = graph.add_edge(vancouver.name.clone(), tokio.name.clone())?;

        // Should return error if node is node in the graph
        assert_eq!(
            graph.traverse_depth_first_iterative("Montreal".to_string()),
            Err("Vertex Montreal does not exist".to_string())
        );

        // Should return the list of explored nodes
        let result = graph.traverse_depth_first_iterative(tokio.name.clone());

        print!("Result {:#?}", result);

        // This can not be tested easily as ordered is not guaranteed due to the
        // use of HashSet to store edges
        // assert_eq!(result, Ok(vec![tokio.name, paris.name, vancouver.name]));

        Ok(())
    }

    // Note: this test is very basic and should be improved
    #[test]
    fn test_traverse_breadth_first() -> Result<(), String> {
        let mut graph: OrientedGraph<City> = OrientedGraph::new();

        // Add cities
        let tokio = City {
            name: "Tokio".to_string(),
        };
        let paris = City {
            name: "Paris".to_string(),
        };
        let vancouver = City {
            name: "Vancouver".to_string(),
        };
        let _ = graph.add_vertex(tokio.clone())?;
        let _ = graph.add_vertex(vancouver.clone())?;
        let _ = graph.add_vertex(paris.clone())?;

        // Add edges
        let _ = graph.add_edge(tokio.name.clone(), vancouver.name.clone())?;
        let _ = graph.add_edge(tokio.name.clone(), paris.name.clone())?;
        let _ = graph.add_edge(vancouver.name.clone(), tokio.name.clone())?;

        // Should return error if node is node in the graph
        assert_eq!(
            graph.traverse_breadth_first("Montreal".to_string()),
            Err("Vertex Montreal does not exist".to_string())
        );

        // Should return the list of explored nodes
        let result = graph.traverse_breadth_first(tokio.name.clone());

        print!("Result {:#?}", result);

        // This can not be tested easily as ordered is not guaranteed due to the
        // use of HashSet to store edges
        // assert_eq!(result, Ok(vec![tokio.name, vancouver.name, paris.name]));

        Ok(())
    }
}
