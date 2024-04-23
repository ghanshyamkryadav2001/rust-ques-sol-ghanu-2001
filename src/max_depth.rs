use std::io;

struct TreeNode {
    val: i32,                         
    left: Option<Box<TreeNode>>,     
    right: Option<Box<TreeNode>>,   
}

pub fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            
            1 + std::cmp::max(max_depth(&node.left), max_depth(&node.right))
        },
        None => {
            
            0
        },
    }
}

pub fn main() {
   
    fn construct_tree() -> Option<Box<TreeNode>> {
        println!("Enter the value of the node (or 0 to skip):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let val: i32 = input.trim().parse().expect("Please enter a valid integer");

        if val == 0 {
            return None;
        }

        println!("Enter the left subtree for node {}:", val);
        let left = construct_tree();

        println!("Enter the right subtree for node {}:", val);
        let right = construct_tree();

        Some(Box::new(TreeNode {
            val,
            left,
            right,
        }))
    }

    println!("Constructing binary tree:");
    let tree = construct_tree();

    println!("Maximum depth of the tree is: {}", max_depth(&tree));
}
