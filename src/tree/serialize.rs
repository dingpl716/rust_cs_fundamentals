// use crate::tree::tree_node::TreeNode;
use crate::tree::TreeNode;
use std::collections::VecDeque;
use std::num::ParseIntError;

const DELIMITER: &str = " ";
const NULL: &str = "#";

pub fn serialize_dfs(root: &TreeNode) -> String {
    let mut result = String::new();
    do_serialize_dfs(root, &mut result);

    return result;
}

fn do_serialize_dfs(root: &TreeNode, result: &mut String) {
    if result.len() != 0 {
        result.push_str(DELIMITER);
    }
    result.push_str(&root.val.to_string());

    if let Some(left) = &root.left {
        do_serialize_dfs(&left, result);
    } else {
        result.push_str(DELIMITER);
        result.push_str(NULL);
    }

    if let Some(right) = &root.right {
        do_serialize_dfs(&right, result);
    } else {
        result.push_str(DELIMITER);
        result.push_str(NULL);
    }
}

pub fn deserialize_dfs(result: &str) -> Result<Option<TreeNode>, ParseIntError> {
    let mut queue: VecDeque<&str> = result.split(DELIMITER).filter(|s| *s != "").collect();
    let root = do_deserialize_dfs(&mut queue);
    return root;
}

fn do_deserialize_dfs(queue: &mut VecDeque<&str>) -> Result<Option<TreeNode>, ParseIntError> {
    match queue.pop_front() {
        None => {
            return Ok(None);
        }
        Some(s) => {
            if s != NULL {
                let val = s.parse::<i32>()?;
                let mut root = TreeNode {
                    val,
                    left: None,
                    right: None,
                };

                if let Some(left) = do_deserialize_dfs(queue)? {
                    root.update_left(Some(Box::new(left)));
                }

                if let Some(right) = do_deserialize_dfs(queue)? {
                    root.update_right(Some(Box::new(right)));
                }

                return Ok(Some(root));
            } else {
                return Ok(None);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::tree_node::TreeNode;

    #[test]
    fn serialize_dfs_works() {
        let node2 = TreeNode {
            val: 2,
            left: None,
            right: None,
        };

        let node3 = TreeNode {
            val: 3,
            left: None,
            right: None,
        };

        let node1 = TreeNode {
            val: 1,
            left: Some(Box::new(node2)),
            right: Some(Box::new(node3)),
        };

        let result = serialize_dfs(&node1);
        assert_eq!("1 2 # # 3 # #", result);
        assert_eq!(1, node1.val);
    }

    #[test]
    fn deserialize_dfs_works() {
        let s = "3 9 # # 20 15 # # 7 # #";
        let root = deserialize_dfs(s).unwrap();
        let ser = serialize_dfs(&root.unwrap());
        assert_eq!(s, ser);
    }

    #[test]
    fn deserialize_dfs_works_for_empty() {
        let s = "";
        let root = deserialize_dfs(s).unwrap();
        assert_eq!(None, root);
    }
}
