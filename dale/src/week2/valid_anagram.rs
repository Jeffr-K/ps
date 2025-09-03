use std::collections::HashMap;

/// [242:Easy:달레스터디(2주차)] Valid Anagram
///
/// link: https://leetcode.com/problems/climbing-stairs/
///
/// constraints:
/// 1 <= s.length, t.length <= 5 * 104
/// s and t consist of lowercase English letters.
/// Tags:
/// WHAT I LEARNED:
/// : 애너그램(Anagram)은, 단어나 문장의 철자를 바꿔서 다른 단어나 문장을 만드는 것이다.
/// : 즉, 같은 알파벳을 다른 순서로 배열해서 다른 뜻의 단어나 문장을 만드는 것을 의미한다.
pub struct Solution;

impl Solution {
    /// TC: O(n)
    /// SC: O(n)
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = HashMap::new();

        for c in s.chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        for ch in t.chars() {
            if let Some(val) = count.get_mut(&ch) {
                if *val == 0 {
                    count.remove(&ch);
                }
            } else {
                return false;
            }
        }

        count.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_anagram() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
        assert!(Solution::is_anagram("".to_string(), "".to_string()));
        assert!(!Solution::is_anagram("a".to_string(), "b".to_string()));
    }
}
