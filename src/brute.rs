use crate::vector::{Vector, euclidean_distance};

pub struct BruteForceIndex {
    vectors: Vec<Vector>,
}

impl BruteForceIndex {
    pub fn new() -> Self {
        Self {
            vectors: Vec::new(),
        }
    }

    pub fn insert(&mut self, v: Vector) -> usize {
        let id = self.vectors.len();
        self.vectors.push(v);
        id
    }

    pub fn search(&self, query: &Vector, k: usize) -> Vec<(usize, f32)> {
        if self.vectors.is_empty() || k == 0 {
            return Vec::new();
        }

        let mut distances: Vec<(usize, f32)> = self
            .vectors
            .iter()
            .enumerate()
            .map(|(id, v)| {
                let dist = euclidean_distance(&query, &v);
                (id, dist)
            })
            .collect();

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        distances.truncate(k);
        distances
    }
}
