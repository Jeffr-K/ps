/// [125; Easy]. Valid Palindrome
///
/// @description
/// - A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters,
///   it reads the same forward and backward. Alphanumeric characters include letters and numbers.
/// @link: https://leetcode.com/problems/valid-palindrome/description/
/// @constraints:
/// - 1 <= s.length <= 2 * 105
/// - s consists only of printable ASCII characters.
/// @tags: [""]
pub struct ValidPalindrome;

impl ValidPalindrome {
    #[allow(dead_code)]
    pub fn is_palindrome(s: String) -> bool {
        let cleaned: Vec<char> = s.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        if cleaned.is_empty() {
            return true;
        }

        cleaned.iter().zip(cleaned.iter().rev()).all(|(a, b)| a == b)
    }

    #[allow(dead_code, non_snake_case)]
    fn isPalindromeRough(s: String) -> bool {
        let mut start = 0;
        let mut end = s.len() - 1;
        let chars: Vec<char> = s.chars().collect();

        while start < end {
            if !chars[start].is_ascii_alphanumeric() {
                start += 1;
                continue;
            } else if !chars[end].is_ascii_alphanumeric() {
                end -= 1;
                continue;
            } else {
                if chars[start].to_ascii_lowercase() != chars[end].to_ascii_lowercase() {
                    return false;
                }
                start += 1;
                end -= 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(ValidPalindrome::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
        assert_eq!(ValidPalindrome::is_palindrome("race a car".to_string()), false);
        assert_eq!(ValidPalindrome::is_palindrome(" ".to_string()), true);
    }

    #[test]
    fn test_is_palindrome_rough() {
        assert_eq!(ValidPalindrome::isPalindromeRough("A man, a plan, a canal: Panama".to_string()), true);
        assert_eq!(ValidPalindrome::isPalindromeRough("race a car".to_string()), false);
        assert_eq!(ValidPalindrome::isPalindromeRough(" ".to_string()), true);
    }
}
