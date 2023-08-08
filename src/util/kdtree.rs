//Subtrees are contained within Box and as Rust does not allow simplly recursive 
// structs as it is unable to know their size at compile time. Containing the
// subtrees within box provides a fixed size pointer.

#[derive(PartialEq)]
pub struct KDTree {
    //point: (u32, u32),
    point: [u32; 2],
    axis: u32,
    depth: u32,
    left_tree: Option<Box<KDTree>>,
    right_tree: Option<Box<KDTree>>,
}

impl KDTree {
    pub fn new(point_list: &Vec<[u32; 2]>, axis: u32, depth: u32) -> KDTree {
        // TODO : Better way to deal with an empty list?
        if point_list.len() == 0 {
            panic!("KDTree cannot be empty");
        }

        let mut new_points: Vec<[u32; 2]> = point_list.clone();
        let axis = axis % depth;

        new_points.sort_unstable();
        let median = new_points.len() / 2;
        let median_node = new_points[median];

        let left_tree;
        if median > 0 && !new_points[..(median - 1)].is_empty() {
            left_tree = Some(Box::new(KDTree::new(&new_points[..(median - 1)].to_vec(), axis + 1, depth)));
        } else {
            left_tree = None;
        }

        let right_tree;
        if (median + 1) < new_points.len() && !new_points[(median + 1)..].is_empty() {
            right_tree = Some(Box::new(KDTree::new(&new_points[(median + 1)..].to_vec(), axis + 1, depth)));
        } else {
            right_tree = None;
        }

        return KDTree { point: median_node, axis: axis, depth: depth, left_tree: left_tree, right_tree: right_tree }
    }

    pub fn add(&mut self, new_point: [u32; 2]) {
        if new_point[self.depth as usize] < self.point[self.depth as usize] {
            match &mut self.left_tree {
                Some(x) => x.add(new_point),
                None => self.left_tree = Some(Box::new(KDTree { point: new_point, 
                                                                axis: (self.axis + 1) % self.depth, 
                                                                depth: self.depth, 
                                                                left_tree: None, 
                                                                right_tree: None })),
            }
        } else {
            match &mut self.right_tree {
                Some(x) => x.add(new_point),
                None => self.right_tree = Some(Box::new(KDTree { point: new_point, 
                                                                 axis: (self.axis + 1) % self.depth, 
                                                                 depth: self.depth, 
                                                                 left_tree: None, 
                                                                 right_tree: None })),
            }
        }
    }

    //Create a vector of all points below some node
    pub fn collect_points_below(&self) -> Vec<[u32; 2]> {
        fn collect_points_below_helper(tree: &Box<KDTree>) -> Vec<[u32; 2]>{
            let mut left_list = match &tree.left_tree {
                Some(x) => collect_points_below_helper(x),
                None => Vec::new()
            };

            left_list.push(tree.point);

            let right_list = match &tree.right_tree {
                Some(x) => collect_points_below_helper(x),
                None => Vec::new()
            };
            return [left_list, right_list].concat()
        }
    
        let mut left_list = match &self.left_tree { 
            Some(x) => collect_points_below_helper(x),
            None => Vec::new(),
        };

        left_list.push(self.point);

        let right_list = match &self.right_tree {
            Some(x) => collect_points_below_helper(x),
            None => Vec::new(),
        };

        let points: Vec<[u32; 2]> = [left_list, right_list].concat();

        return points
    }

    pub fn find(&self, target: [u32; 2]) -> Option<&KDTree> {
        if self.point[0] == target[0] && self.point[1] == target[1] {
            return Some(self)
        } else if self.point[self.depth as usize] <= target[self.depth as usize] {
            return match &self.left_tree {
                Some(left) => left.find(target),
                None => None 
            }
        } else {
            return match &self.right_tree {
                Some(right) => right.find(target),
                None => None
            }
        }
    }

    pub fn find_parent(&self, child: &KDTree) -> Option<&KDTree> {
        //TODO : handle case where child arg is tree root
        if let Some(x) = &self.left_tree {
            if x.point == child.point { return Some(self) }
        } 
        if let Some(x) = &self.right_tree {
            if x.point == child.point { return Some(self) }
        }


        if self.point[self.axis as usize] <= child.point[self.axis as usize] {
            match &self.left_tree {
                Some(x) => x.find_parent(child),
                None => None
            }
        } else {
            match &self.right_tree {
                Some(x) => x.find_parent(child),
                None => None
            }
        }
    }

    pub fn remove(&mut self, target: [u32; 2]) {
        let points_below = self.collect_points_below();

        if let Some(x) = &self.left_tree {
            if x.point == target {
                self.left_tree = Some(Box::new(KDTree::new(&points_below, self.axis, self.depth)))
            }
        } 
        if let Some(x) = &self.right_tree {
            if x.point == target { 
                self.right_tree = Some(Box::new(KDTree::new(&points_below, self.axis, self.depth)))
            }
        }


        if self.point[self.axis as usize] <= target[self.axis as usize] {
            match &mut self.left_tree {
                Some(x) => x.remove(target),
                None => panic!("hi")
            }
        } else {
            match &mut self.right_tree {
                Some(x) => x.remove(target),
                None => panic!("hi")
            }
        }
    }

    pub fn balance() {
        //TODO: Not implemented
    }

    pub fn to_string(&self) {
        println!("hi {}", self.point[0]);

        if let Some(left) = &self.left_tree {
            left.to_string()
        }
        if let Some(right) = &self.right_tree {
            right.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adder() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn make_empty_tree() {
        KDTree::new(&vec![], 0, 2);
    }

    #[test]
    fn make_tiny_tree() {
        let test_tree = KDTree::new(&vec![[0, 0]], 0, 2);
        test_tree.to_string();
        assert_eq!(test_tree.point, [0, 0]);
    }

    #[test]
    fn make_small_tree() {
        let test_tree = KDTree::new(&vec![[0, 0], [1, 0], [2, 0]], 0, 2);

        assert_eq!(test_tree.point, [1, 0]);

        match test_tree.left_tree {
            Some(x) => assert_eq!(x.point, [0, 0]),
            None => assert_eq!(0, 1),
        }

        match test_tree.right_tree {
            Some(x) => assert_eq!(x.point, [2, 0]),
            None => assert_eq!(0, 1)
        }
    }

    #[test]
    fn test_remove_by_point() {
        let test_tree = KDTree::new(&vec![[0, 0], [1, 0], [2, 0]], 0, 2);
        test_tree.remove([2, 0]);

        assert_eq!(test_tree.point, [1, 0]);

        match test_tree.left_tree {
            Some(x) => assert_eq!(x.point, [0, 0]),
            None => assert_eq!(0, 1),
        }

        match test_tree.right_tree {
            None => assert_eq(1, 1),
            Some(x) => assert_eq!(0, 1),
        }
    }
}
