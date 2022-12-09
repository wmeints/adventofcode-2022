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

    println!("Solution part 1: {}", deletable_nodes_sizes);

    let total_size = tree.borrow().total_size();
    let drive_size=  70_000_000;
    let update_size = 30_000_000;
    let unused_size = drive_size - total_size;
    let required_size = update_size - unused_size;

    let all_folders = tree.borrow().flatten();
    
    let mut candidate_folders: Vec<&Rc<RefCell<TreeNode>>> = all_folders.iter()
        .filter(|folder| folder.borrow().total_size() > required_size)
        .collect();

        candidate_folders.sort_by(|a,b| {
        let a_size = a.borrow().total_size();
        let b_size = b.borrow().total_size();

        a_size.cmp(&b_size)
    });

    let folder = candidate_folders.iter().next().unwrap();

    println!("Solution part 2: {}", folder.borrow().total_size());
}

