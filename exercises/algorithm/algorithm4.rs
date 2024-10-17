/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


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

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match self.root {
            Some(ref mut node) => {
                // 如果根节点已经存在，调用 TreeNode 的 insert 递归插入
                node.insert(value);
            }
            None => {
                // 如果根节点为空，插入新的值作为根节点
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match self.root {
            Some(ref node) => node.search(value),
            None => false, // 树为空，直接返回 false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left_node) = self.left {
                    left_node.insert(value); // 递归到左子树
                } else {
                    self.left = Some(Box::new(TreeNode::new(value))); // 插入到左子树
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right_node) = self.right {
                    right_node.insert(value); // 递归到右子树
                } else {
                    self.right = Some(Box::new(TreeNode::new(value))); // 插入到右子树
                }
            }
            Ordering::Equal => {
                // 如果值相同，可以选择忽略插入，或根据应用场景处理
                // 这里我们忽略相同的值
            }
        }
    }
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref left_node) = self.left {
                    left_node.search(value) // 递归到左子树
                } else {
                    false // 左子树为空，值不存在
                }
            }
            Ordering::Greater => {
                if let Some(ref right_node) = self.right {
                    right_node.search(value) // 递归到右子树
                } else {
                    false // 右子树为空，值不存在
                }
            }
            Ordering::Equal => true, // 找到值
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


