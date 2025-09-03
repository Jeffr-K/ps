# Binary Search Tree

### Definition

A Binary Search Tree (BST) is a binary tree where each node has a value, and for any given node:

- The left subtree contains only nodes with values less than the node's value.

- The right subtree contains only nodes with values greater than the node's value.

### Properties

- **Height**: The height of a BST is the number of edges on the longest path from the root to a leaf node.

- **Balanced**: A balanced BST maintains a height that is logarithmic relative to the number of nodes, ensuring efficient operations.

### Operations

- **Insertion**: Insert a new node by comparing its value with the current node's value and placing it in the left or right subtree accordingly.

- **Deletion**: Remove a node by finding it, then rearranging the tree to maintain the BST properties. This may involve replacing the node with its in-order predecessor or successor.

- **Search**: Find a node by comparing its value with the current node's value and traversing left or right accordingly.

### Perfomance

- `Binary Tree` 보다 `insert` 와 `delete` 가 빠르다.
