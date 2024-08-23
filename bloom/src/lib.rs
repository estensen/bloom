use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BloomFilter {
    bit_array: Vec<bool>,
    size: usize,
}

impl BloomFilter {
    pub fn new(size: usize) -> Self {
        Self {
            bit_array: vec![false; size],
            size,
        }
    }

    pub fn insert<T: Hash>(&mut self, item: &T) {
        // Get two bits from hash functions as fingerprint
        let (hash1, hash2) = self.hashes(item);
        self.bit_array[hash1 % self.size] = true;
        self.bit_array[hash2 % self.size] = true;
    }

    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        let (hash1, hash2) = self.hashes(item);
        self.bit_array[hash1 % self.size] && self.bit_array[hash2 % self.size]
    }

    fn hashes<T: Hash>(&self, item: &T) -> (usize, usize) {
        let mut hasher1 = DefaultHasher::new();
        item.hash(&mut hasher1);
        let hash1 = hasher1.finish() as usize;

        let mut hasher2 = DefaultHasher::new();
        item.hash(&mut hasher2);
        // Mix first hash into second hasher
        // to make sure they are different.
        hasher2.write_u64(hash1 as u64);
        let hash2 = hasher2.finish() as usize;

        (hash1, hash2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter_insert_and_contains() {
        let mut bloom_filter = BloomFilter::new(64);

        let item1 = "apple";
        let item2 = "banana";

        bloom_filter.insert(&item1);
        bloom_filter.insert(&item2);

        println!("Contains 'apple': {}", bloom_filter.contains(&item1));
        println!("Contains 'banana': {}", bloom_filter.contains(&item2));
        println!("Contains 'grape': {}", bloom_filter.contains(&"grape"));
    }
}
