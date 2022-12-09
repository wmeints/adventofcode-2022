use std::rc::Rc;
use std::cell::RefCell;
use std::fs;
use day07::{parse_text, build_tree, TreeNode};

fn find_deletable_nodes(tree: &Rc<RefCell<TreeNode>>, expected_size: i64) -> Vec<Rc<RefCell<TreeNode>>> {
    let mut deletable_nodes = Vec::new();

    for child in &tree.borrow().children {
        if child.borrow().total_size() < expected_size {
            deletable_nodes.push(Rc::clone(child));
        } 

        deletable_nodes.append(&mut find_deletable_nodes(child, expected_size));
    }

    deletable_nodes
}

fn main() {
    let input_data = fs::read_to_string("data/input.txt").expect("Can't read input file");
    let commands = parse_text(&input_data);
    let tree = build_tree(&commands);

    let deletable_nodes_sizes: i64 = find_deletable_nodes(&tree, 100_000)
        .iter().map(|node| Rc::clone(node).borrow().total_size())
        .sum();

    println!("Matching folders total size: {}", deletable_nodes_sizes);
}

