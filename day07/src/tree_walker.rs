use crate::parser::Syntax;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    name: String,
    size: Option<i32>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(name: String, size: Option<i32>) -> TreeNode {
        TreeNode {
            name,
            size,
            children: Vec::new(),
            parent: None,
        }
    }

    fn find_child(&self, name: &String) -> Option<Rc<RefCell<TreeNode>>> {
        for child in &self.children {
            if child.borrow().name == *name {
                return Some(Rc::clone(child));
            }
        }

        None
    }

    pub fn total_size(&self) -> i64 {
        let mut total = 0;

        for child in &self.children {
            if let Some(size) = child.borrow().size {
                total += size as i64;
            } else {
                total += child.borrow().total_size();
            }
        }

        total
    }

    fn add_child(&mut self, child: Rc<RefCell<TreeNode>>) {
        self.children.push(child);
    }
}

pub fn build_tree(commands: &Vec<Syntax>) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::new("/".to_string(), None)));
    let mut current = Rc::clone(&root);

    for command in commands {
        match command {
            Syntax::ChangeDirectory { target } => {
                match target.as_str()  {
                    ".." => {
                        let current_clone = Rc::clone(&current);
                        current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                    }
                    "/" => current = Rc::clone(&root),
                    _ => {
                        let current_clone = Rc::clone(&current);
                        let child = current_clone.borrow().find_child(target).unwrap();

                        current = Rc::clone(&child);
                    }
                };
            }
            Syntax::ListContents => {}
            Syntax::Directory { name } => {
                let new_directory = Rc::new(RefCell::new(TreeNode::new(name.to_string(), None)));
                new_directory.borrow_mut().parent = Some(Rc::clone(&current));
                current.borrow_mut().add_child(Rc::clone(&new_directory));
            }
            Syntax::File { name, size } => {
                let new_file = Rc::new(RefCell::new(TreeNode::new(name.to_string(), Some(*size))));
                new_file.borrow_mut().parent = Some(Rc::clone(&current));
                current.borrow_mut().add_child(Rc::clone(&new_file));
            }
        }
    }

    root
}
