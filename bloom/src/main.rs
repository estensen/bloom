fn main() {
    let mut bloom_filter = bloom::BloomFilter::new(64);

    let item1 = "apple";
    let item2 = "banana";

    bloom_filter.insert(&item1);
    bloom_filter.insert(&item2);

    println!("Contains 'apple': {}", bloom_filter.contains(&item1));
    println!("Contains 'banana': {}", bloom_filter.contains(&item2));
    println!("Contains 'grape': {}", bloom_filter.contains(&"grape"));
}
