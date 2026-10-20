use std::collections::HashMap;

pub fn rank(counts: &HashMap<String, u64>, top: usize) -> Vec<(String, u64)> {
    let mut rows: Vec<(String, u64)> = counts
        .iter()
        .map(|(word, count)| (word.clone(), *count))
        .collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    rows.truncate(top);
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(pairs: &[(&str, u64)]) -> HashMap<String, u64> {
        pairs
            .iter()
            .map(|(word, count)| (word.to_string(), *count))
            .collect()
    }

    #[test]
    fn orders_by_count_desc_then_word_asc() {
        let counts = map(&[("b", 2), ("a", 2), ("c", 3)]);
        let rows = rank(&counts, 10);
        let words: Vec<&str> = rows.iter().map(|(word, _)| word.as_str()).collect();
        assert_eq!(words, ["c", "a", "b"]);
    }

    #[test]
    fn truncates_to_the_requested_top_n() {
        let counts = map(&[("a", 1), ("b", 2)]);
        assert_eq!(rank(&counts, 1), vec![("b".to_string(), 2)]);
    }

    #[test]
    fn zero_top_n_yields_no_rows() {
        let counts = map(&[("a", 1)]);
        assert!(rank(&counts, 0).is_empty());
    }
}
