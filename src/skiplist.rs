struct Node {
    value: i32,
    forward: Vec<Option<usize>>,
}

pub struct SkipList {
    nodes: Vec<Node>,
    head_forward: Vec<Option<usize>>,
    max_level: usize,
}

pub fn random_level(max_level: usize) -> usize {
    let mut level = 0;

    while rand::random::<bool>() && level < max_level {
        level += 1;
    }

    level
}

impl SkipList {
    pub fn new(max_level: usize) -> Self {
        Self {
            nodes: Vec::new(),
            head_forward: vec![None; max_level + 1],
            max_level,
        }
    }

    pub fn insert(&mut self, value: i32) {
        let mut update: Vec<Option<usize>> = vec![None; self.max_level +1];

        let mut current: Option<usize> = None;

        for level in (0..=self.max_level).rev() {
            loop {
                let next = match current {
                    None => self.head_forward[level],
                    Some(idx) => self.nodes[idx].forward[level],
                };

                if let Some(next_idx) = next {
                    if self.nodes[next_idx].value < value {
                        current = Some(next_idx);
                        continue;
                    }
                }
                break;
            }
            update[level] = current;
        }

        let new_node_level = random_level(self.max_level);

        let new_node_idx = self.nodes.len();
        let new_node = Node {
            value,
            forward: vec![None; new_node_level + 1],
        };
        self.nodes.push(new_node);

        for level in 0..=new_node_level {
            match update[level] {
                None => {
                    self.nodes[new_node_idx].forward[level] = self.head_forward[level];
                    self.head_forward[level] = Some(new_node_idx);
                }
                Some(prev_idx) => {
                    self.nodes[new_node_idx].forward[level] = self.nodes[prev_idx].forward[level];
                    self.nodes[prev_idx].forward[level] = Some(new_node_idx);
                }
            }
        }
    }

    pub fn search(&self, value:i32) -> bool {
        let mut current: Option<usize> = None;

        for level in (0..=self.max_level).rev() {
            loop {
                let next = match current {
                    None => self.head_forward[level],
                    Some(idx) => self.nodes[idx].forward[level],
                };

                match next {
                    Some(idx) if self.nodes[idx].value < value => {
                        current = Some(idx);
                    }
                    _ => break,
                }
            }
        }

        let final_next = match current {
            None => self.head_forward[0],
            Some(idx) => self.nodes[idx].forward[0],
        };

        match final_next {
            Some(idx) => self.nodes[idx].value == value,
            None => false,
        }
    }
}