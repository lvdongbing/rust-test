// 请从命令行读取一行字符串（例如 "apple banana pear banana apple banana"）。
// 使用空格进行拆分，统计每个单词出现的次数，并以从高到底的顺序输出。
// 如果出现次数相同，按单词本身的字典序从小到大排序输出。

pub fn question3() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "apple banana pear banana apple banana".to_string());
    for (word, count) in deal_string(input.as_str()) {
        println!("{}: {}", word, count);
    }
}

fn deal_string(input: &str) -> Vec<(String, usize)> {
    let mut word_count = std::collections::HashMap::new();

    for word in input.split_whitespace() {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }

    let mut sorted_words: Vec<_> = word_count.iter().collect();
    sorted_words.sort_by(|a, b| {
        let count_cmp = b.1.cmp(a.1);
        if count_cmp == std::cmp::Ordering::Equal {
            a.0.cmp(b.0)
        } else {
            count_cmp
        }
    });

    sorted_words.into_iter().map(|(k, v)| (k.clone(), *v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question3() {
        let input = "apple banana pear banana apple banana";
        let result = deal_string(input);
        let expected = vec![
            ("banana".to_string(), 3),
            ("apple".to_string(), 2),
            ("pear".to_string(), 1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = deal_string(input);
        let expected: Vec<(String, usize)> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_word() {
        let input = "apple";
        let result = deal_string(input);
        let expected = vec![("apple".to_string(), 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_words_with_same_count() {
        let input = "apple banana apple banana";
        let result = deal_string(input);
        let expected = vec![
            ("apple".to_string(), 2),
            ("banana".to_string(), 2),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_words_with_same_count_sorted_alphabetically() {
        let input = "banana apple pear";
        let result = deal_string(input);
        let expected = vec![
            ("apple".to_string(), 1),
            ("banana".to_string(), 1),
            ("pear".to_string(), 1),
        ];
        assert_eq!(result, expected);
    }
}