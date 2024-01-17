use std::{cmp::{max, min}, rc::Rc};

#[derive(Clone)]
pub struct SegmentTree {
    pub left_bound: usize,
    pub right_bound: usize,
    pub sum: i32,
    pub children: Option<(Rc<SegmentTree>, Rc<SegmentTree>)>,
    pub test: Option<Rc<SegmentTree>>,
}

impl SegmentTree {
    pub fn new(left_bound: usize, right_bound: usize) -> SegmentTree {
        let mut tree = SegmentTree {
            left_bound,
            right_bound,
            sum: 0,
            children: None,
            test: None,
        };
        if left_bound + 1 < right_bound {
            let t = (left_bound + right_bound) / 2;
            let left_child = SegmentTree::new(left_bound, t);
            let right_child = SegmentTree::new(t, right_bound);
            tree.children = Some((Rc::new(left_child), Rc::new(right_child)));
        }
        tree
    }

    pub fn add(&mut self, position: usize, value: i32) {
        self.sum += value;

        if let Some((ref mut left_child, ref mut right_child)) = self.children.as_mut() {
            if left_child.left_bound <= position && position < left_child.right_bound {
                let mut new_left_child = (**left_child).clone();
                // new_left_child.sum += value;
                new_left_child.add(position, value);
                *left_child = Rc::new(new_left_child);
            }
            else {
                let mut new_right_child = (**right_child).clone();
                // new_right_child.sum += value;
                new_right_child.add(position, value);
                *right_child = Rc::new(new_right_child);
            }   
        }
    }

    pub fn sum(&self, left_bound: usize, right_bound: usize) -> i32 {
        // node bounds are inside query bounds
        if left_bound <= self.left_bound && self.right_bound <= right_bound {
            return self.sum
        }
        // node and query bounds does not overlap
        if max(left_bound, self.left_bound) >= min(right_bound, self.right_bound) {
            return 0
        }
        // complex: let children decide
        if let Some((left_child, right_child)) = &self.children {
            return left_child.sum(left_bound, right_bound) + right_child.sum(left_bound, right_bound)
        }
        unreachable!()
    }
}

#[derive(Default)]
pub struct PersistentSegmentTree {
    pub roots: Vec<SegmentTree>,
}

impl PersistentSegmentTree {
    pub fn new(left_bound: usize, right_bound: usize) -> Self {
        PersistentSegmentTree { roots: vec![SegmentTree::new(left_bound, right_bound)] }
    }

    pub fn add(&mut self, position: usize, value: i32, version: usize) {
        let mut root = self.roots[version].clone();
        root.add(position, value);
        self.roots.push(root); 
    }
}


fn main() {
    let mut tree = PersistentSegmentTree::new(1, 9);
    tree.add(2, 1, 0);
    tree.add(4, 1, 1);
    tree.add(5, 1, 2);
    tree.add(7, 1, 3);
    tree.add(8, 10, 4);

    let l = 1;
    let r = 9;
    for i in 0..6 {
        println!(
            "version {}, sum on [{}, {}) is {}",
            i,
            l,
            r,
            tree.roots[i].sum(l, r));
    }    
}

