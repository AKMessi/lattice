mod brute;
mod skiplist;
mod vector;

use brute::BruteForceIndex;
use vector::Vector;
use skiplist::SkipList;

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

    println!("\n=== Stage 2: Skip List ===");
    let mut list = SkipList::new(4);

    // insert in a genuinely random (non-sorted) order — this matters,
    // since a skip list's structure depends on insertion order
    // interacting with the random level draws
    let values = vec![50, 10, 90, 30, 70, 20, 80, 40, 60];
    for &v in &values {
        list.insert(v);
    }

    // search for values known to be present
    for &v in &values {
        assert!(list.search(v), "expected to find {} but didn't", v);
    }

    // search for values known to be absent
    for missing in [5, 15, 25, 100, 0] {
        assert!(
            !list.search(missing),
            "found {} but it was never inserted",
            missing
        );
    }

    println!("Stage 2: OK — all present values found, all absent values correctly not found");
}
