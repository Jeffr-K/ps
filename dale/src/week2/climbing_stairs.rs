use std::collections::HashMap;

/// [70:Easy] Climbing Stairs
///
/// link: https://leetcode.com/problems/climbing-stairs/
/// constraints:
/// -1 <= n <= 45
/// TAGS: Dynamic Programming, Recursion, Memoization, Fibonacci Sequence
/// WHAT I LEARNED:
pub struct Solution;

impl Solution {
    /// TC: O(2^n)
    /// SC: O(n) - call stack
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2)
    }

    /// TC: O(n)
    /// SC: O(n)
    /// HashMap을 사용한 Top-down DP (Memoization)
    pub fn use_dp(n: i32) -> i32 {
        let mut memo = HashMap::new();
        memo.insert(0, 1);
        memo.insert(1, 1);

        for i in 2..=n {
            let val = memo.get(&(i - 1)).unwrap() + memo.get(&(i - 2)).unwrap();
            memo.insert(i, val);
        }

        *memo.get(&n).unwrap()
    }

    /// TC: O(n)
    /// SC: O(1)
    /// 반복문으로 계산, 공간 최적화
    pub fn optimized(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        let (mut pre, mut cur) = (1, 2);

        for _ in 3..=n {
            let next = pre + cur;
            pre = cur;
            cur = next;
        }

        cur
    }

    /// TC: O(n)
    /// SC: O(1)
    /// fold 를 이용한 함수형 스타일
    /// T.fold 를 사용하려면 `iterator`.`into_iter()` 또는 `iter()` 를 호출한 후에 사용해야 한다.
    /// fold() 는 이터레이터의 모든 요소를 순회하면서 누적값을 계산하는 함수로, 초기값과 클로저를 인자로 받는다.
    /// ```rust
    /// fn fold<B, F>(
    ///     self,
    ///     init: B,
    ///     f: F
    /// ) -> B
    ///      where B: FromIterator<Self::Item>,
    ///            F: FnMut(B, Self::Item) -> B
    /// ```
    /// - self: 이터레이터
    /// - init: 초기값
    /// - f: 누적값을 계산하는 클로저
    /// - return: 최종 누적값
    #[allow(non_snake_case)]
    pub fn useFoldStyle(n: i32) -> i32 {
        match n {
            0 | 1 => n,
            2 => 2,
            _ => (3..=n).fold((1, 2), |(pre, cur), _| (cur, pre + cur)).1,
        }
    }

    /// TC: O(n)
    /// SC: O(n)
    /// scan을 이용해 상태를 누적하며 마지막 값을 가져오는 함수형 스타일
    #[allow(non_snake_case)]
    pub fn useScanStyle(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        (3..=n)
            .scan((1, 2), |state, _| {
                *state = (state.1, state.0 + state.1);
                Some(state.1)
            })
            .last()
            .unwrap()
    }

    /// TC: O(n)
    /// SC: O(1)
    /// successors를 이용한 완전한 이터레이터 기반 함수형 스타일
    #[allow(non_snake_case)]
    pub fn useSuccessorStyle(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        std::iter::successors(Some((1, 2)), |&(pre, cur)| Some((cur, pre + cur)))
            .map(|(_, cur)| cur)
            .nth((n - 2) as usize)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_all(n: i32, expected: i32) {
        assert_eq!(Solution::climb_stairs(n), expected);
        assert_eq!(Solution::use_dp(n), expected);
        assert_eq!(Solution::optimized(n), expected);
        assert_eq!(Solution::useFoldStyle(n), expected);
        assert_eq!(Solution::useScanStyle(n), expected);
        assert_eq!(Solution::useSuccessorStyle(n), expected);
    }

    #[test]
    fn test_all_versions() {
        assert_all(2, 2);
        assert_all(3, 3);
        assert_all(4, 5);
        assert_all(5, 8);
        assert_all(6, 13);
        assert_all(10, 89);
    }
}
