class Node:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None


class BinarySearchTree:

    def __init__(self, root: int):
        self.root = Node(root)

    def insert(self, value: int):
        self._insert_recursive(self.root, value)

    def _insert_recursive(self, node: Node, value: int):
        if value < node.data:
            if node.left is None:
                node.left = Node(value)
            else:
                self._insert_recursive(node.left, value)
        else:
            if node.right is None:
                node.right = Node(value)
            else:
                self._insert_recursive(node.right, value)

    def delete(self, value: int):
        self.root = self._delete_recursive(self.root, value)

    def _delete_recursive(self, node: Node, value: int) -> Node:
        if node is None:
            return None

        if value < node.data:
            node.left = self._delete_recursive(node.left, value)
        elif value > node.data:
            node.right = self._delete_recursive(node.right, value)
        else:
            # 1. 리프 노드
            if node.left is None and node.right is None:
                return None
            # 2. 왼쪽 또는 오른쪽만 있는 노드
            elif node.left is None:
                return node.right
            elif node.right is None:
                return node.left
            # 3. 양쪽 자식이 다 있는 경우
            else:
                # 오른쪽 서브트리에서 가장 작은 값으로 교체(Successor)
                successor = self._min_value_node(node.right)
                node.data = successor.data
                node.right = self._delete_recursive(node.right, successor.data)

                # 왼쪽 서브트리에서 가장 큰 값으로 교체(Predecessor)
                # predecessor = self._max_value_node(node.left)
                # node.value = predecessor.value
                # node.left = self._delete(node.left, predecessor.value)

        return node

    def clear(self):
        if self.root is None:
            return
        self.root.data = None
        self.root.left = None
        self.root.right = None

    def search(self, node: Node, value: int):
        if node is None or node.data == value:
            return node
        elif value < node.data:
            return self.search(node.left, value)
        else:
            return self.search(node.right, value)

    def update(self, value: int):
        pass

    def inorder(self, node: Node):
        if node:
            self.inorder(node.left)
            print(node.data, end=' ')
            self.inorder(node.right)

    def preorder(self, node: Node):
        if node is None:
            return

        print(node.data, end=' ')
        self.preorder(node.left)
        self.preorder(node.right)

    def postorder(self, node: Node):
        if node is None:
            return

        self.postorder(node.left)
        self.postorder(node.right)
        print(node.data, end=' ')

    def levelorder(self, node):
        from queue import Queue

        if node is None:
            return

        q = Queue()
        q.put(node)

        while not q.empty():
            current = q.get()
            print(current.data, end=' ')

            if current.left:
                q.put(current.left)
            if current.right:
                q.put(current.right)

    def _min_value_node(self, node: Node) -> Node:
        """최소값 노드를 찾는 헬퍼 메소드(Successor)"""
        current = node
        while current.left is not None:
            current = current.left
        return current

    def _max_value_node(self, node: Node) -> Node:
        """최대값 노드를 찾는 헬퍼 메소드(Predecessor)"""
        current = node
        while current.right is not None:
            current = current.right
        return current

    def __str__(self):
        # Optional: 트리의 중위 순회 결과를 문자열로 반환
        from io import StringIO
        output = StringIO()

        def collect(node):
            if node:
                collect(node.left)
                output.write(str(node.data) + ' ')
                collect(node.right)

        collect(self.root)
        return output.getvalue().strip()


def __main__():
    bst = BinarySearchTree(root=70)
    bst.insert(10)
    bst.insert(90)
    bst.insert(80)
    bst.insert(20)

    print("Inorder Traversal (by method):")
    bst.inorder(bst.root)

    print("\nPreorder Traversal (by method):")
    bst.preorder(bst.root)

    print("\nPostorder Traversal (by method):")
    bst.postorder(bst.root)

    print("\nLevel Order Traversal (by method):")
    bst.levelorder(bst.root)

    print("\nSearching for 20:")
    found_node = bst.search(bst.root, 20)
    if found_node:
        print(f"Found: {found_node.data}")
    else:
        print("Not found")
    print("\nSearching for 100:")
    found_node = bst.search(bst.root, 100)
    if found_node:
        print(f"Found: {found_node.data}")
    else:
        print("Not found")

    print("\nInorder Traversal (by __str__):")
    print(bst)

    print("\nDeleting 10:")
    bst.delete(10)
    print("Inorder Traversal after deletion:")
    bst.inorder(bst.root)
    print("\nDeleting 70 (root):")
    bst.delete(70)
    print("Inorder Traversal after deletion:")
    bst.inorder(bst.root)


if __name__ == "__main__":
    __main__()
