/// [39; Medium] Combination Sum
///
/// @description: Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.
///               The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers is different.
///               The test cases are generated such that the number of unique combinations that sum up to target is less than 150 combinations for the given input.
/// @link: https://leetcode.com/problems/combination-sum/
/// @constraints:
/// - 1 <= nums.length <= 105
/// - -104 <= nums[i] <= 104
/// @tags: ["Backtracking"]
/// @approach:
/// - `candidates` 배열에 있는 숫자들을 중복 사용해서 합이 `target` 이 되는 모든 숫자 조합을 찾는 문제.
pub struct CombinationSum;

impl CombinationSum {
    #[allow(dead_code)]
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current_combination = Vec::new();

        // 백트래킹을 위한 재귀 함수 호출
        Self::backtrack(
            &mut result,
            &mut current_combination,
            &candidates,
            target,
            0, // 탐색을 시작할 인덱스
        );

        result
    }

    /// 재귀적으로 조합을 탐색하는 백트래킹 함수
    fn backtrack(
        result: &mut Vec<Vec<i32>>,          // 최종 결과를 담을 벡터
        current_combination: &mut Vec<i32>,  // 현재 만들고 있는 조합
        candidates: &Vec<i32>,               // 사용할 수 있는 숫자 목록
        remain: i32,                         // 남은 목표 합계
        start_index: usize,                  // 탐색 시작 위치
    ) {
        // 목표 합계를 정확히 맞춘 경우: 성공
        if remain == 0 {
            result.push(current_combination.clone());
            return;
        }

        // 목표 합계를 초과한 경우: 실패(이전 단계로 돌아감)
        if remain < 0 {
            return;
        }

        // 현재 탐색 위치부터 후보 숫자들을 순회
        for i in start_index..candidates.len() {
            let candidate = candidates[i];

            // 1. 선택 (Choose): 현재 후보 숫자를 조합에 추가
            current_combination.push(candidate);

            // 2. 탐색 (Explore): 남은 목표 합계로 다음 단계 재귀 호출
            //    start_index를 `i`로 넘겨서 현재 숫자를 중복해서 사용할 수 있게 함
            Self::backtrack(result, current_combination, candidates, remain - candidate, i);

            // 3. 되돌리기 (Unchoose): 재귀 호출이 끝나면, 선택했던 숫자를 다시 제거
            //    그래야 다음 반복에서 새로운 조합을 만들 수 있음
            current_combination.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        // LeetCode에서는 결과의 순서를 신경쓰지 않으므로, 테스트를 위해 정렬이 필요할 수 있습니다.
        // 하지만 assert_eq!는 순서까지 비교하므로, 실제 제출 시에는 정렬이 필요 없습니다.
        let mut res1 = CombinationSum::combination_sum(vec![2, 3, 6, 7], 7);
        res1.iter_mut().for_each(|v| v.sort());
        res1.sort();
        assert_eq!(res1, vec![vec![2, 2, 3], vec![7]]);

        let mut res2 = CombinationSum::combination_sum(vec![2, 3, 5], 8);
        res2.iter_mut().for_each(|v| v.sort());
        res2.sort();
        assert_eq!(res2, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);

        assert_eq!(CombinationSum::combination_sum(vec![2], 1), Vec::<Vec<i32>>::new());
    }
}
