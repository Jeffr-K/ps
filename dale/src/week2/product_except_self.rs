/// [238::Medium::달레스터디(2주차)] Product of Array Except Self
///
/// :각 인덱스 i에 대해, `nums[i]` 를 제외한 모든 값의 곱을 `answer[i]` 에 저장하는 배열을 구하는 문제.
///
/// /// [link](https://leetcode.com/problems/product-of-array-except-self/)
///
///
/// Constraints:
/// - 2 <= nums.length <= 10^5
/// - -30 <= nums[i] <= 30
/// The input is generated such that answer[i] is guaranteed to fit in a 32-bit integer.
///
/// Tags:
///
/// WHAT I LEARNED:
///
///
#[allow(dead_code)]
pub struct Solution;

impl Solution {
    // Time Limit Exceeded
    // TC: O(n^2)
    // SC: O(n)
    // Approach:
    // 1. Edge case: If the input array is empty, return an empty array.
    // 2. Edge case: If the input array contains only two elements, return them in reversed order.
    // 2. `nums` 의 개수가 반환 벡터의 크기를 결정하기 때문에 clone 을 사용해서 `answer` 를 초기화한다.
    // 3. `nums[i]` 를 제외한 모든 값의 곱을 nums[i] 의 위치에 저장해야 한다. 이 때, 모든 값의 곱은 순차적으로 연산한다.
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![nums[1], nums[0]];
        }

        if nums.is_empty() {
            return vec![];
        }

        let mut answer: Vec<i32> = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            let mut product = 1;

            for j in 0..nums.len() {
                if i != j {
                    product *= nums[j];
                }
            }

            answer.push(product);
        }

        answer
    }

    // TC: O(n)
    // SC: O(n)
    // Approach:
    // 1. 왼쪽에서부터의 곱을 저장하는 `left` 변수를 사용하여, 각 인덱스 i에 대해 `nums[0]` 부터 `nums[i-1]` 까지의 곱을 계산한다.
    // 2. 오른쪽에서부터의 곱을 저장하는 `right` 변수를 사용하여, 각 인덱스 i에 대해 `nums[i+1]` 부터 `nums[n-1]` 까지의 곱을 계산한다.
    // 3. `answer[i]` 는 `left` 와 `right` 의 곱으로 계산된다.
    // Usecase:
    // 1. 데이터 분석: 특정 데이터 포인트를 제외한 나머지 데이터 집합의 통계치를 계산할 때;
    // 2. 머신러닝/딥러닝: 배치 처리에서 특정 요소를 제외한 조합 계산 할 때;
    // 3. 게임 개발: 특정 캐릭터의 능력치를 제외한 나머지 캐릭터들의 능력치를 계산할 때;
    // Learn:
    // - Rust 에서 Vec<T> 는 이미 스마트 포인터로 구현되어 있어, `answer` 변수 자체가 Vec<i32> 라는 소유권이 있는 컬렉션으로 초기화 된다는 사실이다.
    // - 반면, HashMap<T, U> 는 키와 값의 쌍을 저장하는 해시 테이블로, 소유권이 있는 컬렉션이지만, `Key` 나 `Value` 를 꺼낼 떄에는 참조자(Reference)인 `&T` 나 `&U` 를 사용해야 한다는 점이다.
    // - 예를 들어, `HashMap::get(&key)` 는 `&T` 타입의 참조자를 반환한다. 이는 소유권이 아닌 참조를 통해 값을 가져온다는 것을 의미한다.
    // Retrospectives:
    // 이 문제는 배열의 각 인덱스에 대해 해당 인덱스를 제외한 나머지 요소들의 곱을 계산하는 문제로, 효율적인 접근법을 사용하여 O(n) 시간 복잡도로 해결할 수 있다.
    // 유형 느낌 자체가, 특정 요소를 제외한 배열의 나머지 요소들의 곱을 계산하는 문제로 파악하면 될 것 같다.
    #[allow(non_snake_case)]
    pub fn usePrefixProduct(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![1; nums.len()];

        let mut left = 1;

        for i in 0..nums.len() {
            answer[i] = left;
            left *= nums[i];
        }

        let mut right = 1;

        for i in (0..nums.len()).rev() {
            answer[i] *= right;
            right *= nums[i];
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_product_except_self() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(Solution::product_except_self(vec![0, 0]), vec![0, 0]);
        assert_eq!(Solution::product_except_self(vec![3, 7]), vec![7, 3]);
        assert_eq!(Solution::product_except_self(vec![]), vec![]);
    }
}
