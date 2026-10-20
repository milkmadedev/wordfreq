pub fn tokenize(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(text: &str) -> Vec<String> {
        tokenize(text).collect()
    }

    #[test]
    fn splits_on_non_alphanumeric_characters() {
        assert_eq!(words("hello, world!"), ["hello", "world"]);
    }

    #[test]
    fn lowercases_tokens() {
        assert_eq!(words("The THE the"), ["the", "the", "the"]);
    }

    #[test]
    fn skips_punctuation_only_tokens() {
        assert!(words("... !! ??").is_empty());
    }

    #[test]
    fn digit_runs_form_tokens() {
        assert_eq!(words("a42 007x"), ["a42", "007x"]);
    }
}
