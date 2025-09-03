use std::cell::RefCell;
use std::rc::Rc;

/// [98::Medium::달레스터디(2주차)] Validate Binary Search Tree
///
/// :Binary Tree 의 `RootNode` 가 주어졌을 때, 그것이 유효한 이진 검색 트리(Binary Search Tree, BST)인지 확인하는 문제.
/// :유효한 BST 는 다음 조건을 만족해야 한다:
/// :- 모든 왼쪽 서브트리의 노드 값은 현재 노드의 값보다 작아야 한다.
/// :- 모든 오른쪽 서브트리의 노드 값은 현재 노드의 값보다 커야 한다.
/// :- 왼쪽과 오른쪽 서브트리도 각각 유효한 BST 여야 한다.
///
/// *Binary Search Tree*
/// :
///
/// /// [link](https://leetcode.com/problems/validate-binary-search-tree/description/)
///
/// Constraints:
/// The number of nodes in the tree is in the range [1, 10^4].
/// -2^31 <= Node.val <= 2^31 - 1
///
/// Tags:
///
/// WHAT I LEARNED:
/// :- `Rust` 에서 `crate` 는 패키지의 기본 단위로, `프로젝트` 또는 `컴파일 단위` 를 의미한다.
/// :- Crate 의 종류로서, `Binary Crate` | `Library Crate` | `Executable Crate` 가 있다.
/// :- 1) 코드 재사용성과 모듈화 명확성을 향상시킬 수 있다.
/// :- 2) 큰 프로그램을 작고 독립적인 단위(Crate)로 나누어 관리하면, 빌드 속도 향상, 의존성 관리 용이성, 코드 재사용성 등의 이점을 얻을 수 있다.
/// :- 3) Rustc 가 각 Crate 를 독립적+병렬적으로 컴파일할 수 있어, 전체 프로그램의 빌드 시간을 단축시킬 수 있다.
/// :- 4) unwrap() 은 런타임 에러를 발생시킬 수 있으므로, 안전한 코드 작성을 위해 unwrap() 사용을 피하는 것이 좋다.
/// :- 5) 대체제로 if let 구문을 사용하여, Option 또는 Result 타입의 값을 안전하게 처리할 수 있다.
pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 현재 값을 저장할 변수(Memoization)
        // 현재 노드의 값과 좌/우 노드의 값을 비교하여 BST 조건을 만족했는지 확인한다.
        // 이때, 순회 방식은 중위 순회(In-order Traversal)를 사용한다.
        // 중위 순회는 왼쪽 서브트리 -> 현재 노드 -> 오른쪽 서브트리 순서로 노드를 방문한다.
        // 이 순서를 통해 BST의 특성을 유지하면서 노드를 순회할 수 있다.
        Self::useInOrderTraversal(&root, None, None)
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn useInOrderTraversal(
        node: &Option<Rc<RefCell<TreeNode>>>,
        lower: Option<i32>,
        upper: Option<i32>,
    ) -> bool {
        if let Some(n) = node {
            let n_ref = n.borrow();
            let val = n_ref.val;

            // 상한값 검증
            if let Some(up) = upper {
                if val >= up {
                    return false;
                }
            }

            // 하한값 검증
            if let Some(low) = lower {
                if val <= low {
                    return false;
                }
            }

            // 왼쪽 노드 탐색 (val 보다 작아야 하므로 upper = Some(val))
            if !Self::useInOrderTraversal(&n_ref.left, lower, Some(val)) {
                return false;
            }

            // 오른쪽 노드 탐색 (val 보다 커야 하므로 lower = Some(val))
            if !Self::useInOrderTraversal(&n_ref.right, Some(val), upper) {
                return false;
            }

            true
        } else {
            // node == None 이면 재귀 종료; 호출 상위 스택으로 복귀
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::Solution;
    use super::TreeNode;

    #[test]
    fn test_is_valid_bst() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert!(Solution::is_valid_bst(root));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        })));
        assert!(!Solution::is_valid_bst(root));
    }
}
