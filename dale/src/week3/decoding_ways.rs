/// [91; Medium] Decoidng Ways
///
/// @description: You have intercepted a secret message encoded as a string of numbers. The message is decoded via the following mapping:
///               - '1' -> 'A'
///               - '2' -> 'B'
///               - '3' -> 'C'
///               - ...
///               - '26' -> 'Z'
///
///               However, while decoding the message, you realize that there are many different ways you can decode the message because some codes are contained in other codes ("2" and "5" vs "25").
///
///               For example, "11106" can be decoded into:
///               - "AAJF" with the grouping (1, 1, 10, 6)
///               - "KJF" with the grouping (11, 10, 6)
///               - The grouping (1, 11, 06) is invalid because "06" is not a valid code (only "6" is valid).
///
///               Note: there may be strings that are impossible to decode.
///
///               Given a string s containing only digits, return the number of ways to decode it. If the entire string cannot be decoded in any valid way, return 0.
///               The test cases are generated so that the answer fits in a 32-bit integer.
/// @link: https://leetcode.com/problems/decode-ways/description/
/// @constraints:
/// - -1 <= n <= 45
/// @tags: ["Dynamic Programming"]
/// @approach
/// - 순서는 고정된 채 숫자를 어떻게 그룹으로 묶을 것인가에 대한 경우의 수를 찾는 문제
/// - 일단 이해가 안감
pub struct DecodingWays;

impl DecodingWays {
    #[allow(dead_code)]
    pub fn num_decodings(s: String) -> i32 {
        if s.starts_with('0') {
            return 0;
        }

        // DP 상태를 저장할 변수. two_back은 dp[i-2], one_back은 dp[i-1] 역할
        let mut two_back = 1; // dp[0]에 해당 (빈 문자열)
        let mut one_back = 1; // dp[1]에 해당 (첫 번째 문자)

        for i in 1..s.len() {
            let mut current = 0;
            let s_bytes = s.as_bytes();

            // 1. 한 글자만 보는 경우 (예: "226"에서 '6')
            // '0'이 아니면, 이전까지의 경우의 수를 그대로 이어갈 수 있음.
            // 왜 as_bytes()를 사용하는지 설명
            if s_bytes[i] != b'0' {
                current += one_back;
            }

            // 2. 두 글자를 함께 보는 경우 (예: "226"에서 '26')
            // '10'부터 '26' 사이의 유효한 값인지 확인
            let two_digits = (s_bytes[i - 1] - b'0') * 10 + (s_bytes[i] - b'0');
            if (10..=26).contains(&two_digits) {
                current += two_back;
            }

            // 다음 반복을 위해 DP 상태를 업데이트
            two_back = one_back;
            one_back = current;
        }

        one_back
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_decodings() {
        assert_eq!(DecodingWays::num_decodings("12".to_string()), 2);
        assert_eq!(DecodingWays::num_decodings("226".to_string()), 3);
        assert_eq!(DecodingWays::num_decodings("0".to_string()), 0);
        assert_eq!(DecodingWays::num_decodings("10".to_string()), 1);
        assert_eq!(DecodingWays::num_decodings("27".to_string()), 1);
    }
}
