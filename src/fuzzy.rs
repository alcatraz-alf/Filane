/// Fuzzy subsequence matching ala fzf.
/// Mengembalikan Some(score) jika pattern cocok (subsequence case-insensitive),
/// None jika tidak cocok. Score lebih tinggi = match lebih baik.
pub fn fuzzy_match(text: &str, pattern: &str) -> Option<i64> {
    if pattern.is_empty() {
        return Some(0);
    }

    let text_lower = text.to_lowercase();
    let text_chars: Vec<char> = text_lower.chars().collect();
    let pattern_chars: Vec<char> = pattern.to_lowercase().chars().collect();

    let mut score: i64 = 0;
    let mut t_idx = 0usize;
    let mut last_match: Option<usize> = None;

    for &p in &pattern_chars {
        // Find next occurrence of p in text starting from t_idx
        let mut found = None;
        while t_idx < text_chars.len() {
            if text_chars[t_idx] == p {
                found = Some(t_idx);
                break;
            }
            t_idx += 1;
        }
        let idx = found?;
        t_idx += 1;

        // Scoring
        score += 10; // base per matched char
        if let Some(prev) = last_match {
            if idx == prev + 1 {
                score += 15; // consecutive bonus
            }
        }
        if idx == 0 {
            score += 20; // start-of-string bonus
        } else {
            let prev_char = text_chars[idx - 1];
            if prev_char == '/' || prev_char == '_' || prev_char == '-' || prev_char == '.' || prev_char == ' ' {
                score += 10; // boundary bonus
            }
        }
        last_match = Some(idx);
    }

    // Penalty for length gap (prefer tighter matches)
    score -= (text_chars.len() as i64 - pattern_chars.len() as i64) / 4;

    Some(score)
}

/// Check if a filename matches a filter pattern.
/// Plain substring match first (fast path), then fuzzy.
pub fn filter_match(name: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }
    let name_lower = name.to_lowercase();
    let pattern_lower = pattern.to_lowercase();

    // Fast path: exact substring
    if name_lower.contains(&pattern_lower) {
        return true;
    }
    // Fallback: fuzzy subsequence
    fuzzy_match(name, pattern).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substring_match() {
        assert!(filter_match("Cargo.toml", "cargo"));
        assert!(filter_match("Cargo.toml", "toml"));
    }

    #[test]
    fn fuzzy_match_works() {
        assert!(filter_match("Cargo.toml", "ctm")); // C-a-r-g-o.T-o-m-l subsequence
        assert!(filter_match("MyDocumentFile.txt", "mdf"));
        assert!(!filter_match("Cargo.toml", "xyz"));
    }

    #[test]
    fn empty_pattern_matches_all() {
        assert!(filter_match("anything", ""));
    }
}
