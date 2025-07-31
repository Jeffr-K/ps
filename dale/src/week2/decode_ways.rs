#[allow(dead_code)]
struct Solution;

impl Solution {
    /// 91. Decode Ways
    /// :You have intercepted a secret message encoded as a string of numbers. The message is decoded via the following mapping:
    /// ⏳TC:
    /// 💾SC: O(n)
    /// 💬Approach:
    /// 💬WHAT I LEARNED:
    /// 💬TAGS:
    pub fn num_decodings(s: String) -> i32 {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_decodings() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        assert_eq!(Solution::num_decodings("0".to_string()), 0);
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
        assert_eq!(Solution::num_decodings("27".to_string()), 1);
    }
}
