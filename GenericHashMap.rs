use std::collections::HashMap;
use std::collections::BTreeMap; // For sorting

// Define a generic HashMap struct
struct MyHashMap<K, V> {
    map: HashMap<K, V>,
}

// Define a trait called SortByKey
trait SortByKey<K, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)>;
}

// Implement the SortByKey trait for MyHashMap
impl<K: Ord, V> SortByKey<K, V> for MyHashMap<K, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)> {
        let mut sorted_map: BTreeMap<&K, &V> = BTreeMap::new();

        for (key, value) in &self.map {
            sorted_map.insert(key, value);
        }

        sorted_map.iter().map(|(k, v)| (*k, *v)).collect()
    }
}

fn main() {
    // Create a new instance of MyHashMap
    let mut my_map = MyHashMap {
        map: HashMap::new(),
    };

    // Add some key-value pairs to the map
    my_map.map.insert("C", 3);
    my_map.map.insert("A", 1);
    my_map.map.insert("B", 2);

    // Sort the elements in the map by their keys
    let sorted_pairs = my_map.sort_by_key();

    // Print the sorted key-value pairs
    for (key, value) in sorted_pairs {
        println!("Key: {:?}, Value: {:?}", key, value);
    }
}
