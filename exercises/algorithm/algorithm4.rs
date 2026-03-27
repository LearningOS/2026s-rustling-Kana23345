/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::arch::naked_asm;
use std::arch::x86_64::_SIDD_LEAST_SIGNIFICANT;
use std::cmp::{self, Ordering};
use std::fmt::Debug;
use std::ptr::null;

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
        match &mut self.root {
            None => self.root = Option::Some(Box::new(TreeNode::new(value))),
            Some(n) => n.insert(value),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match &self.root {
            None => false,
            Some(node) => node.search(value),
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn search(&self, value: T) -> bool {
        match self.value.cmp(&value) {
            Ordering::Greater => match &self.left {
                Some(left) => left.search(value),
                None => false,
            },
            Ordering::Less => match &self.right {
                Some(right) => right.search(value),
                None => false,
            },
            Ordering::Equal => true,
        }
    }
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match self.value.cmp(&value) {
            Ordering::Greater => match &mut self.left {
                Some(left) => {
                    left.insert(value);
                }
                None => {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            },
            Ordering::Less => match &mut self.right {
                Some(right) => {
                    right.insert(value);
                }
                None => {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            },
            Ordering::Equal => {}
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
