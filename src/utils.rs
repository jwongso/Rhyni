use std::collections::HashMap;

pub fn split_and_normalize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| word.chars()
            .filter(|c| c.is_alphanumeric())
            .collect())
        .filter(|word: &String| !word.is_empty())
        .collect()
}

pub fn merge_strings(a: &str, b: &str) -> String {
    let base = split_and_normalize(a);
    let tail = split_and_normalize(b);
    let mut index_map: HashMap<String, Vec<usize>> = HashMap::new();

    for (i, word) in base.iter().enumerate() {
        index_map.entry(word.clone()).or_default().push(i);
    }

    let mut merge_point = None;

    for (i, word) in tail.iter().enumerate() {
        if let Some(indices) = index_map.get(word) {
            for &idx in indices {
                if idx + 2 < base.len() && i + 2 < tail.len() {
                    if base[idx + 1] == tail[i + 1] && base[idx + 2] == tail[i + 2] {
                        merge_point = Some(idx);
                        break;
                    }
                }
            }
        }
        if merge_point.is_some() {
            break;
        }
    }

    // Merge texts
    let mut result = base.clone();
    if let Some(idx) = merge_point {
        result.truncate(idx);
    }
    result.extend(tail);
    result.join(" ")
}

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_and_normalize() {
        let result = split_and_normalize("Hello, world! This is Rust.");
        assert_eq!(result, vec!["Hello", "world", "This", "is", "Rust"]);
    }

    #[test]
    fn test_merge_strings() {
        let a = "The quick brown fox jumps over the lazy dog.";
        let b = "brown fox jumps over the fence.";
        let merged = merge_strings(a, b);
        assert_eq!(merged, "The quick brown fox jumps over the fence.");
    }
}
