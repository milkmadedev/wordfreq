use std::collections::HashMap;

pub fn count<I>(words: I) -> HashMap<String, u64>
where
    I: IntoIterator<Item = String>,
{
    let mut counts: HashMap<String, u64> = HashMap::new();
    for word in words {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counted(words: &[&str]) -> HashMap<String, u64> {
        count(words.iter().map(|word| word.to_string()))
    }

    #[test]
    fn counts_repeated_words() {
        let counts = counted(&["a", "b", "a"]);
        assert_eq!(counts["a"], 2);
        assert_eq!(counts["b"], 1);
    }

    #[test]
    fn single_occurrences_count_once() {
        let counts = counted(&["solo"]);
        assert_eq!(counts.get("solo"), Some(&1));
        assert_eq!(counts.len(), 1);
    }
}
