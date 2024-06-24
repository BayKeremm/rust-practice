use std::cmp::Ordering;

#[derive(Debug)]
pub struct Node {
    pub ch: char,
    pub freq: f32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    // Constructor method
    pub fn new(ch: char, freq: f32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
        Node { ch, freq, left, right }
    }
}

// Implementing PartialOrd for comparison
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.freq.partial_cmp(&other.freq)
    }
}

// Implementing Ord for total ordering
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.freq.is_nan() {
            if other.freq.is_nan() {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else if other.freq.is_nan() {
            Ordering::Greater
        } else {
            self.freq.partial_cmp(&other.freq).unwrap_or(Ordering::Equal)
        }
    }
}


// Implementing PartialEq for equality
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

// Implementing Eq for total equality
impl Eq for Node {}
