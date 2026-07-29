mod vector;
mod brute;

use brute::BruteForceIndex;
use vector::Vector;

fn random_vector(dim: usize) -> Vector {
    (0..dim).map(|_| rand::random::<f32>()).collect()
}

fn main() {
    let mut index = BruteForceIndex::new();

    for _ in 0..1000 {
        index.insert(random_vector(128));
    }

    let query = random_vector(128);
    let results = index.search(&query, 5);

    println!("top 5 nearest to query:");
    for (id, dist) in &results {
        println!("  id={} distance={:.4}", id, dist);
    }

    for w in results.windows(2) {
        assert!(w[0].1 <= w[1].1, "results not sorted correctly");
    }

    assert_eq!(results.len(), 5, "expected exactly 5 results for k=5");

    println!("Stage 1: OK");
}