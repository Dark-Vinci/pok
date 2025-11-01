mod clone_graph;

struct Solution;

#[derive(Clone)]
struct Node {
    value: i32,
    neighbors: Vec<Option<Node>>,
}

impl Node {
    fn new(value: i32, neighbors: Vec<Option<Node>>) -> Self {
        Self { value, neighbors }
    }
    
}