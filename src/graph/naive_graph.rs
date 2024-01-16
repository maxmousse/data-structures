use std::collections::HashMap;

pub trait Identifiable {
    fn get_id(&self) -> String;
}

/// A basic graph implementation
///
/// It is 'naive' in the sens that it plays around reference and lifetime rust safety
/// by using String id as pointers for the vertices hashmap keys, and cloned string
/// to refer the vertices in the edges.
/// It skips all the safety rust can bring, forces to do runtime check and to be really careful
/// when cleaning vertices/edges in order to keep the graph correct
///
/// To benefit of rust safety, more advanced memory management methods should be used
/// (see [here](https://stackoverflow.com/questions/34747464/implement-graph-like-data-structure-in-rust)
/// and [here](https://github.com/nrc/r4cppp/blob/master/graphs/README.md) for more infos)
#[derive(Debug)]
pub struct Graph<T: Identifiable> {
    pub vertices: HashMap<String, T>,
    pub edges: HashMap<String, Vec<String>>,
}

impl<T: Identifiable> Graph<T> {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: T) -> Result<(), String> {
        let vertex_id = vertex.get_id();

        if self.vertex_exists(&vertex_id) {
            Err("Vertex ".to_owned() + &vertex_id + " already exists")
        } else {
            self.vertices.insert(vertex_id.clone(), vertex);
            self.edges.insert(vertex_id, vec![]);
            Ok(())
        }
    }

    pub fn add_edge(&mut self, vertex1_id: String, vertex2_id: String) -> Result<(), String> {
        // Check that vertices exist, or err
        self.vertex_exists_or_err(&vertex1_id)?;
        self.vertex_exists_or_err(&vertex2_id)?;

        // Insert the new edge
        self.edges.get_mut(&vertex1_id).unwrap().push(vertex2_id);

        Ok(())
    }

    pub fn remove_edge(&mut self, vertex1_id: String, vertex2_id: String) -> Result<(), String> {
        // Check that vertices exist, or err
        self.vertex_exists_or_err(&vertex1_id)?;
        self.vertex_exists_or_err(&vertex2_id)?;

        self.edges
            .get_mut(&vertex1_id)
            .and_then(|ids| {
                ids.iter()
                    .position(|elem| elem == &vertex2_id)
                    .map(|index| ids.remove(index))
            })
            .ok_or(
                "Edge from vertex ".to_owned()
                    + &vertex1_id
                    + " to vertex "
                    + &vertex2_id
                    + " does not exist",
            )
            .map(|_| ())
    }

    pub fn remove_vertex(&mut self, vertex_id: String) -> Result<(), String> {
        // Check that vertex exist, or err
        self.vertex_exists_or_err(&vertex_id)?;

        // Remove edges from this vertex
        self.edges.remove(&vertex_id);

        // Remove edges to this vertex
        for vec in self.edges.values_mut() {
            vec.retain(|elem| *elem != vertex_id);
        }

        // Remove the vertex
        self.vertices.remove(&vertex_id);

        Ok(())
    }

    pub fn vertex_exists(&self, id: &String) -> bool {
        self.vertices.contains_key(id)
    }

    pub fn vertex_exists_or_err(&self, id: &String) -> Result<(), String> {
        if self.vertex_exists(id) {
            Ok(())
        } else {
            Err("Vertex ".to_owned() + id + " does not exist")
        }
    }

    pub fn get_vertex_ref(&self, id: &str) -> Option<&T> {
        self.vertices.get(id)
    }

    pub fn get_vertex_ref_or_err(&self, id: &str) -> Result<&T, String> {
        self.get_vertex_ref(id)
            .ok_or("Vertex with id ".to_owned() + id + " does not exists")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // basic struct for testing purpose
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
        let graph: Graph<City> = Graph::new();

        assert_eq!(graph.vertices, HashMap::new());
        assert_eq!(graph.edges, HashMap::new());
    }

    #[test]
    fn test_add_vertex() {
        let mut graph: Graph<City> = Graph::new();

        let tokio = City {
            name: "Tokio".to_string(),
        };

        // Adding a city should work
        let mut result = graph.add_vertex(tokio.clone());

        assert_eq!(result, Ok(()));
        assert_eq!(graph.vertices.get("Tokio"), Some(&tokio));

        // Adding the same city should Err
        result = graph.add_vertex(tokio);

        assert_eq!(result, Err("Vertex Tokio already exists".to_string()))
    }

    #[test]
    fn test_add_edge() {
        let mut graph: Graph<City> = Graph::new();

        let tokio = City {
            name: "Tokio".to_string(),
        };

        let vancouver = City {
            name: "Vancouver".to_string(),
        };

        // Add cities
        let _ = graph.add_vertex(tokio.clone());
        let _ = graph.add_vertex(vancouver.clone());

        // Adding the edge should work
        let mut result = graph.add_edge(tokio.name.clone(), vancouver.name.clone());

        assert_eq!(result, Ok(()));
        assert_eq!(
            graph.edges.get(&tokio.name),
            Some(&vec![vancouver.name.clone()])
        );

        // Adding an edge with an absent city should Err
        result = graph.add_edge(tokio.name.clone(), "Montreal".to_string());

        assert_eq!(result, Err("Vertex Montreal does not exist".to_string()))
    }

    #[test]
    fn test_remove_edge() {
        let mut graph: Graph<City> = Graph::new();

        // Add cities
        let tokio = City {
            name: "Tokio".to_string(),
        };
        let vancouver = City {
            name: "Vancouver".to_string(),
        };
        let _ = graph.add_vertex(tokio.clone());
        let _ = graph.add_vertex(vancouver.clone());
        let _ = graph.add_edge(tokio.name.clone(), vancouver.name.clone());

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
        assert!(graph.edges.get(&tokio.name).unwrap().is_empty())
    }

    #[test]
    fn test_remove_vertex() {
        let mut graph: Graph<City> = Graph::new();

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
        let _ = graph.add_vertex(tokio.clone());
        let _ = graph.add_vertex(vancouver.clone());
        let _ = graph.add_edge(tokio.name.clone(), vancouver.name.clone());
        let _ = graph.add_edge(tokio.name.clone(), paris.name.clone());
        let _ = graph.add_edge(vancouver.name.clone(), tokio.name.clone());

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
    }
}
