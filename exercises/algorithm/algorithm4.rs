/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

/*
TreeNode表示二叉搜索树中的一个节点，包含三个字段：
value：节点的值，类型为泛型T，要求T实现Ord trait（即可比较大小）。
left：左子节点，类型为Option<Box<TreeNode<T>>>，表示可能存在的左子树。
right：右子节点，类型为Option<Box<TreeNode<T>>>，表示可能存在的右子树。
*/
#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

/*
BinarySearchTree表示整个二叉搜索树，包含一个字段：
root：树的根节点，类型为Option<Box<TreeNode<T>>>，表示可能存在的根节点。
where特征约束T必须实现Ord trait，以便于比较大小。
*/
impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /*
        1、如果树为空（root为None），则创建一个新的TreeNode作为根节点。
        2、如果树不为空，则递归调用TreeNode的insert方法
    */
    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if let Some(ref mut root) = self.root {
            root.insert(value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }
    /*
        1、如果树为空（root为None），则直接返回false。
        2、如果树不为空，则递归调用TreeNode的search方法
    */
    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        if let Some(ref root) = self.root {
            root.search(value)
        } else {
            false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    /*
    1、需要实现将值插入当前节点下的逻辑。根据值的大小决定插入左子树还是右子树。
    2、如果值小于当前节点的值，则递归插入左子树。
    3、如果值大于当前节点的值，则递归插入右子树。
    4、如果值等于当前节点的值，可以选择不插入（或处理重复值的情况）。

    */
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {}
        }
    }
    /*
        1、需要实现在当前节点下搜索值的逻辑。根据值的大小决定搜索左子树还是右子树。
        2、如果值等于当前节点的值，返回true。
        3、如果值小于当前节点的值，递归搜索左子树。
        4、如果值大于当前节点的值，递归搜索右子树。
        5、如果未找到值，返回false。
    */
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => {
                if let Some(ref left) = self.left {
                    left.search(value)
                } else {
                    false
                }
            }
            Ordering::Greater => {
                if let Some(ref right) = self.right {
                    right.search(value)
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
