struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {}
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_combination_sum() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let result = Solution::combination_sum(candidates, target);
        assert_eq!(result, vec![vec![2, 2, 3], vec![7]]);

        let candidates = vec![2, 3, 5];
        let target = 8;
        let result = Solution::combination_sum(candidates, target);
        assert_eq!(result, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }
}
