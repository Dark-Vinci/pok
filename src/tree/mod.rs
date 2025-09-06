mod has_path_sum;
mod symmetricity;

struct Solution;

#[derive(Debug, Clone)]
struct Tree<T> {
    value: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

impl<T> Tree<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            right: None,
            left: None,
        }
    }
}