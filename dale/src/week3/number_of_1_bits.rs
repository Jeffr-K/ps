/// [191:Easy] Number of 1 Bits
///
/// @description: Given a positive integer n, write a function that returns the number of set bits in its binary representation (also known as the Hamming weight).
/// @ref Hamming weight: https://en.wikipedia.org/wiki/Hamming_weight 데이터에서 1의 개수를 세는 알고리즘
/// @ref [Youtube: Number of 1 Bits](youtube.com/watch?time_continue=3&v=DDMvIb9y3-I&embeds_referring_euri=https%3A%2F%2Fleetcode.com%2F&source_ve_path=MTM5MTE3LDI4NjY2)
/// @link: https://leetcode.com/problems/number-of-1-bits/description/
/// @constraints:
/// - 1 <= n <= 231 - 1
/// @tags: [""]
#[derive(Debug)]
pub struct NumberOf1Bits;

impl NumberOf1Bits {

    #[allow(dead_code)]
    pub fn hamming_weight(n: i32) -> i32 {
        format!("{:b}", n)
            .chars()
            .filter(|&c| c == '1')
            .count() as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(NumberOf1Bits::hamming_weight(11), 3);
        assert_eq!(NumberOf1Bits::hamming_weight(128), 1);
        assert_eq!(NumberOf1Bits::hamming_weight(2147483645), 30);
    }
}
