/// [53; medium] Maximum Subarray
///
/// @link https://leetcode.com/problems/maximum-subarray/description/
/// @description: Given an integer array nums, find the subarray with the largest sum, and return its sum.
/// @ref: A `subarray` is a contiguous non-empty sequence of elements within an array.
/// @ref 순열이 아닌 이유: 순열은 [1,2,3]이 있을 때 [1, 3, 2], [2,1,3] 처럼 순서를 바꾸어 만들 수 있는 모든 경우의 수를 다루는 문제
/// @ref 카데인 알고리즘
/// @ref 이 문제는 배열의 순서는 고정되어 있고, 그 안에서 연속된 구간을 찾는 문제이므로, 보통 동적 계획법(Dynamic Programming) 또는 특정 알고리즘 문제 유형으로 분류
pub struct MaximumSubarray;

impl MaximumSubarray {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut current_sum = nums[0];

        for &num in nums.iter().skip(1) {
            current_sum = current_sum.max(0) + num;
            max_sum = max_sum.max(current_sum);
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(MaximumSubarray::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(MaximumSubarray::max_sub_array(vec![1]), 1);
        assert_eq!(MaximumSubarray::max_sub_array(vec![5,4,-1,7,8]), 23);
    }
}
