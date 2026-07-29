pub type Vector = Vec<f32>;

pub fn euclidean_distance(a: &Vector, b: &Vector) -> f32 {
    assert_eq!(a.len(), b.len(), "vectors must be equal length");

    let sum_squared_diff: f32 = a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum();

    sum_squared_diff.sqrt()
}

pub fn dot_product(a: &Vector, b: &Vector) -> f32 {
    assert_eq!(a.len(), b.len(), "vectors must be equal length");

    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

pub fn cosine_distance(a: &Vector, b: &Vector) -> f32 {
    assert_eq!(a.len(), b.len(), "vectors must be equal length");

    let dot = dot_product(a, b);
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|y| y * y).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    1.0 - (dot / (norm_a * norm_b))
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        assert!((euclidean_distance(&a, &b) - 5.0).abs() < 1e-6); // 3-4-5 triangle
    }

    #[test]
    fn test_cosine_distance_identical_vectors() {
        let a = vec![1.0, 2.0, 3.0];
        assert!(cosine_distance(&a, &a).abs() < 1e-6); // identical vectors -> distance 0
    }

    #[test]
    fn test_cosine_distance_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        assert!((cosine_distance(&a, &b) - 1.0).abs() < 1e-6); // orthogonal -> distance 1
    }
}
