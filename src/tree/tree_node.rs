#[derive(Debug, PartialEq)]
pub struct TreeNode {
    // val: &'a str,
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self {
        Self { val, left, right }
    }

    pub fn update_left(&mut self, left: Option<Box<TreeNode>>) {
        self.left = left;
    }

    pub fn update_right(&mut self, right: Option<Box<TreeNode>>) {
        self.right = right;
    }
}
