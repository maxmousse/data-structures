use std::u32::MAX;
use std::{
    cmp,
    collections::{BinaryHeap, HashMap, HashSet},
};

use super::identifiable::Identifiable;

/// This is the same Graph than `OrientedGraph` except that
/// it is weighted
#[derive(Debug)]
pub struct OrientedWeightedGraph<T: Identifiable> {
    /// List of the graph vertices
    pub vertices: HashMap<String, T>,

    /// List of the graph edges
    ///
    /// Note: The way it is structured allow to express oriented weighted edges
    /// - A -> B by storing the entry { A: { B: weight } }
    /// - B -> A by storing the entry { B: { A: weight } }
    pub edges: HashMap<String, HashMap<String, u32>>,
}

impl<T: Identifiable> OrientedWeightedGraph<T> {
    /// Instantiate a new graph
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
            self.edges.insert(vertex_id, HashMap::new());
            Ok(())
        }
    }

    /// Add an edge into the graph, and Err if it already exists, of if it targets invalid vertices
    pub fn add_edge(
        &mut self,
        vertex1_id: String,
        vertex2_id: String,
        weight: u32,
    ) -> Result<(), String> {
        // Check that vertices exist, or err
        self.vertex_exists_or_err(&vertex1_id)?;
        self.vertex_exists_or_err(&vertex2_id)?;

        // Insert the new edge
        let edge_exists = self
            .edges
            .get(&vertex1_id)
            .unwrap()
            .contains_key(&vertex2_id);

        match edge_exists {
            false => {
                self.edges
                    .get_mut(&vertex1_id)
                    .unwrap()
                    .insert(vertex2_id, weight);
                Ok(())
            }
            true => Err("Edge between from vertex ".to_owned()
                + &vertex1_id
                + " to vertex "
                + &vertex2_id
                + " already exists"),
        }
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

    /// Return the shortest path between two vertices of the graph
    ///
    /// This function uses [Dijkstra's shortest path algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
    pub fn shortest_path(
        &self,
        source_id: String,
        target_id: String,
    ) -> Result<Vec<String>, String> {
        // Check that source and vertices exist, or err
        self.vertex_exists_or_err(&source_id)?;
        self.vertex_exists_or_err(&target_id)?;

        // Keep track of the vertices visited in the main loop
        let mut seen_vertices = HashSet::new();
        // Keep track of the shorter distance between source and a given vertex
        let mut shortest_distances_from_source = HashMap::new();
        // Keep track of the shortest ways between source and a given vertex
        // (This is stored as a linked list)
        let mut path: HashMap<String, String> = HashMap::new();
        // The priority queue is used to iterate step by step between vertices
        // while always priorising the known vertex which is the closest from the
        // source node
        let mut priority_queue = BinaryHeap::new();
        let mut result = Vec::new();

        // Initialize the state with the knowledge we have
        // - The distance between source and itself is 0
        // - Source is the closes node we know
        shortest_distances_from_source.insert(source_id.clone(), 0);
        priority_queue.push(VertexByDistance {
            id: source_id.clone(),
            distance: 0,
        });

        // Loop through the vertices by "closest from the source" order
        while let Some(VertexByDistance {
            id: current_id,
            distance: source_current_distance,
        }) = priority_queue.pop()
        {
            // Mark the current_vertex as seen
            seen_vertices.insert(current_id.clone());

            // If we reach the target vertex, we stop
            // Note: it could be possible to explore the whole
            // graph to return a shortest-path tree
            if current_id == target_id {
                result.push(target_id.clone());
                break;
            }

            // Else, for each neighbor, check if the current path is shorter
            // than a path explored previously, and update the state
            for (neighbor_id, current_neighbor_distance) in self.edges.get(&current_id).unwrap() {
                // Ignore neighbor vertices that have already been visited
                if seen_vertices.contains(neighbor_id) {
                    continue;
                }

                // distance from source->neighbor = distance from source->current + current->neighbor
                let source_neighbor_distance = current_neighbor_distance + source_current_distance;

                if source_neighbor_distance
                    < *shortest_distances_from_source
                        .get(neighbor_id)
                        .unwrap_or(&MAX)
                {
                    // Update shortest distance from source, as a shorter path was found
                    shortest_distances_from_source
                        .insert(neighbor_id.clone(), source_neighbor_distance);

                    // Update path with the shortest path that was found
                    path.insert(neighbor_id.clone(), current_id.clone());

                    // Insert the newly visited vertex in the priority queue
                    // The next closer vertex from the source will be visited first
                    priority_queue.push(VertexByDistance {
                        id: neighbor_id.clone(),
                        distance: source_neighbor_distance,
                    });
                }
            }
        }

        // The loop ends when the targe is reached or all
        // paths were explored and none was find
        // Next, build the path to target vertex by linearize
        // the `path` hashmap
        let mut curr_id = target_id.clone();
        while let Some(next_id) = path.get(&curr_id) {
            result.push(next_id.clone());
            curr_id = next_id.clone();
        }

        // Reverse the result, as it first come backward
        result.reverse();

        return Ok(result);
    }
}

/// Type wrapper that implements Ord and Eq
///
/// It is used to be able to compare vertices by distance and
/// sort them with a BinaryHeap priority queue
struct VertexByDistance {
    id: String,
    distance: u32,
}

impl PartialEq for VertexByDistance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for VertexByDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.cmp(&other.distance).reverse())
    }
}

impl Ord for VertexByDistance {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for VertexByDistance {}

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
        let graph: OrientedWeightedGraph<City> = OrientedWeightedGraph::new();

        assert_eq!(graph.vertices, HashMap::new());
        assert_eq!(graph.edges, HashMap::new());
    }

    #[test]
    fn test_add_vertex() -> Result<(), String> {
        let mut graph: OrientedWeightedGraph<City> = OrientedWeightedGraph::new();

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
        let mut graph: OrientedWeightedGraph<City> = OrientedWeightedGraph::new();

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
        let mut result = graph.add_edge(tokio.name.clone(), vancouver.name.clone(), 10);
        assert_eq!(result, Ok(()));
        assert_eq!(
            graph.edges.get(&tokio.name).unwrap().get(&vancouver.name),
            Some(&10)
        );

        // Adding an edge with an absent city should Err
        result = graph.add_edge(tokio.name.clone(), "Montreal".to_string(), 10);
        assert_eq!(result, Err("Vertex Montreal does not exist".to_string()));

        // Adding an existing edge should Err
        result = graph.add_edge(tokio.name.clone(), vancouver.name.clone(), 10);
        assert_eq!(
            result,
            Err("Edge between from vertex Tokio to vertex Vancouver already exists".to_string())
        );

        Ok(())
    }

    #[test]
    fn test_shortest_path() -> Result<(), String> {
        let mut graph: OrientedWeightedGraph<City> = OrientedWeightedGraph::new();

        // Add cities
        let a = City {
            name: "a".to_string(),
        };
        let b = City {
            name: "b".to_string(),
        };
        let c = City {
            name: "c".to_string(),
        };
        let d = City {
            name: "d".to_string(),
        };
        let e = City {
            name: "e".to_string(),
        };
        let f = City {
            name: "f".to_string(),
        };

        graph.add_vertex(a.clone())?;
        graph.add_vertex(b.clone())?;
        graph.add_vertex(c.clone())?;
        graph.add_vertex(d.clone())?;
        graph.add_vertex(e.clone())?;
        graph.add_vertex(f.clone())?;

        graph.add_edge("a".to_string(), "b".to_string(), 4)?;
        graph.add_edge("a".to_string(), "c".to_string(), 2)?;
        graph.add_edge("b".to_string(), "e".to_string(), 3)?;
        graph.add_edge("c".to_string(), "d".to_string(), 2)?;
        graph.add_edge("c".to_string(), "f".to_string(), 4)?;
        graph.add_edge("d".to_string(), "e".to_string(), 3)?;
        graph.add_edge("d".to_string(), "f".to_string(), 1)?;
        graph.add_edge("f".to_string(), "e".to_string(), 1)?;

        // Validate a simple path
        assert_eq!(
            graph.shortest_path("a".to_string(), "b".to_string()),
            Ok(vec!["a".to_string(), "b".to_string(),])
        );

        // Validate complex path
        assert_eq!(
            graph.shortest_path("a".to_string(), "e".to_string()),
            Ok(vec![
                "a".to_string(),
                "c".to_string(),
                "d".to_string(),
                "f".to_string(),
                "e".to_string(),
            ])
        );

        // Validate no path
        assert_eq!(
            graph.shortest_path("b".to_string(), "a".to_string()),
            Ok(vec![])
        );

        Ok(())
    }
}
