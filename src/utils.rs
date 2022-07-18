use std::collections::HashMap;
use std::hash::Hash;

pub fn to_hashmap_keys<K, V, F>(list: &Vec<K>, map_key_to_value: F) -> HashMap<K, V>
where
    F: Fn(&K) -> V,
    K: Eq + Hash + Clone,
{
    list.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.clone(), map_key_to_value(item));

        acc
    })
}

pub fn merge_hashmaps_with<K, V, F>(
    source: HashMap<K, V>,
    mut target: HashMap<K, V>,
    merge_fn: F,
) -> HashMap<K, V>
where
    F: Fn(V, V) -> V,
    K: Eq + Clone,
    K: std::hash::Hash,
    V: Clone,
{
    for (key, mut value) in source {
        let right_value = target.get(&key);

        if let Some(r_value) = right_value {
            value = merge_fn(value, r_value.clone());
        }

        target
            .entry(key)
            .and_modify(|v| *v = value.clone())
            .or_insert(value);
    }

    target
}

pub fn count_nested<V>(list_of_lists: &Vec<Vec<V>>) -> HashMap<V, u32>
where
    V: Eq + Hash + Clone,
{
    list_of_lists
        .iter()
        .fold(HashMap::new(), |total_counts: HashMap<V, u32>, words| {
            let current_counts = count_all(words);

            merge_hashmaps_with(current_counts, total_counts, |l, r| l + r)
        })
}

pub fn count_all<V>(list: &Vec<V>) -> HashMap<V, u32>
where
    V: Eq + Hash + Clone,
{
    list.iter().fold(HashMap::new(), |mut counts, item| {
        *counts.entry(item.clone()).or_default() += 1;

        counts
    })
}

#[cfg(test)]
mod tests {
    use crate::utils::*;
    use itertools::Itertools;
    use std::collections::HashMap;

    #[test]
    fn test_merge_hashmaps_with() {
        let mut left = HashMap::new();
        left.insert("one", 1);
        left.insert("two", 1);
        left.insert("three", 2);

        let mut right = HashMap::new();
        right.insert("two", 1);
        right.insert("three", 1);
        right.insert("four", 4);

        let mut expected = HashMap::new();
        expected.insert("one", 1);
        expected.insert("two", 2);
        expected.insert("three", 3);
        expected.insert("four", 4);

        let actual = merge_hashmaps_with(left, right, |left, right| left + right);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_all_words() {
        let word_list: Vec<String> = (vec!["chicago", "is", "cold", "africa", "is", "hot"])
            .iter()
            .map(|word| word.to_string())
            .collect();

        let mut expected_counts: HashMap<String, u32> = HashMap::new();
        expected_counts.insert("chicago".to_string(), 1);
        expected_counts.insert("is".to_string(), 2);
        expected_counts.insert("cold".to_string(), 1);
        expected_counts.insert("africa".to_string(), 1);
        expected_counts.insert("hot".to_string(), 1);

        let actual_counts = count_all(&word_list);

        assert_eq!(expected_counts, actual_counts);
    }

    #[test]
    fn test_count_all_bigrams() {
        let word_list: Vec<(String, String)> = (vec!["chicago", "is", "cold", "is", "cold"])
            .iter()
            .map(|word| word.to_string())
            .tuple_windows()
            .collect();

        let mut expected_counts: HashMap<(String, String), u32> = HashMap::new();
        expected_counts.insert(("chicago".to_string(), "is".to_string()), 1);
        expected_counts.insert(("is".to_string(), "cold".to_string()), 2);
        expected_counts.insert(("cold".to_string(), "is".to_string()), 1);

        let actual_counts = count_all(&word_list);

        assert_eq!(expected_counts, actual_counts);
    }
}
