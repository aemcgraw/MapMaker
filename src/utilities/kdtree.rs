//Subtrees are contained within Box and as Rust does not allow simplly recursive 
// structs as it is unable to know their size at compile time. Containing the
// subtrees within box provides a fixed size pointer.
use std::collections::HashMap;

use crate::map_data::MapData;

#[derive(PartialEq)]
pub struct KDTree {
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

        new_points.sort_by(|a, b| a[axis as usize].cmp(&b[axis as usize]));
        let median = new_points.len() / 2;
        let median_node = new_points[median];

        let left_tree;
        if median > 0 && !new_points[..median].is_empty() {
            left_tree = Some(Box::new(KDTree::new(&new_points[..median].to_vec(), axis + 1, depth)));
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

    //Create a vector of all points below the calling node
    // If include_top is true, the point value of the calling node will be included also
    pub fn collect_points(&self, include_top: bool) -> Vec<[u32; 2]> {
        let mut left_list = match &self.left_tree {
            Some(x) => x.collect_points(include_top),
            None => Vec::new()
        };

        if include_top == true {
            left_list.push(self.point);
        }

        let mut right_list = match &self.right_tree {
            Some(x) => x.collect_points(include_top),
            None => Vec::new()
        };

        left_list.append(&mut right_list);
        
        return left_list

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

    pub fn nearest_neighbor(&self, target: [u32; 2], nearest: [u32; 2]) -> [u32; 2] {
        fn distance(x: [u32; 2], y: [u32; 2]) -> f64 {
            let x0 = x[0] as i32;
            let x1 = x[1] as i32;
            let y0 = y[0] as i32;
            let y1 = y[1] as i32;
            return (((x0 - y0).pow(2) + (x1 - y1).pow(2)) as f64).sqrt()
        }

        let side; 
        let c_axis = self.axis as usize;

        // Walk down the tree to a leaf, set this as a candidate solution
        let mut new_nearest = if target[c_axis] <= self.point[c_axis] {
            side = 0;
            match &self.left_tree { 
                Some(x) => x.nearest_neighbor(target, nearest),
                None => {
                    if distance(target, nearest) < distance(target, self.point) { nearest }
                    else { self.point }
                }
            }
        } else {
            side = 1;
            match &self.right_tree {
                Some(x) => x.nearest_neighbor(target, nearest),
                None => {
                    if distance(target, nearest) < distance(target, self.point) { nearest }
                    else { self.point }
                }
            }
        };
 
        //Walk back up the tree, checking if each node is nearer than the best known
        if distance(target, new_nearest) > distance(target, self.point) {
            new_nearest = self.point;
        }

        //If the distance between the target point and the nearest node is greater than the distance
        // between the target and the splitting plane, better solutions may exist on the other side of the plane
        // so we traverse the other side of the tree in search of a better solution
        if distance(target, new_nearest) > ((target[c_axis] as i32) - (self.point[c_axis] as i32)).abs() as f64 {
            if side == 1 {
                match &self.left_tree {
                    Some(x) => return x.nearest_neighbor(target, new_nearest),
                    None => return new_nearest,
                }
            } else {
                match &self.right_tree {
                    Some(x) => return x.nearest_neighbor(target, new_nearest),
                    None => return new_nearest,
                }
            }
        }

        return new_nearest;
    }

    pub fn remove(&mut self, target: [u32; 2]) {
        if let Some(x) = &self.left_tree {
            if x.point == target {
                let points_below = x.collect_points(false);
                if points_below.len() > 0 {
                    self.left_tree = Some(Box::new(KDTree::new(&points_below, self.axis, self.depth)));
                } else {
                    self.left_tree = None;
                }
                return
            }
        } 
        if let Some(x) = &self.right_tree {
            if x.point == target {
                let points_below = x.collect_points(false);
                if points_below.len() > 0 {
                    self.right_tree = Some(Box::new(KDTree::new(&points_below, self.axis, self.depth)));
                } else {
                    self.right_tree = None;
                }
                return
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

    pub fn to_mapdata(&self, width: u32, height: u32) -> MapData {
        let points = self.collect_points(true);
        let mut points_hash: HashMap<[u32; 2], f64> = HashMap::new();

        let mut i = 0.0;
        for x in &points {
            points_hash.insert(*x, i / (points.len() as f64));

            i = i + 1.0;
        }

        let mut data_vec: Vec<f64> = Vec::with_capacity((width * height) as usize);

        for x in 0..width {
            for y in 0..height {
                if points_hash.contains_key(&[x, y]) {
                    data_vec.push(0.05);
                } else {
                    let nearest = self.nearest_neighbor([x, y], self.point);
                    let val = points_hash.get(&nearest).unwrap();
                    data_vec.push(val.clone());
                }
            }
        }

        return MapData::new(data_vec, width, height);
    }

    pub fn pretty_print(&self) {
        fn point_to_string(x: [u32; 2]) -> String {
            let mut this_string = String::from("[ ");
            this_string.push_str(&x[0].to_string());
            this_string.push_str(" , ");
            this_string.push_str(&x[1].to_string());
            this_string.push_str(" ] ");
            return this_string;
        }

        let mut self_string = point_to_string(self.point);

        if let Some(x) = &self.left_tree {
            self_string.push_str(&point_to_string(x.point));
            x.pretty_print();
        } else {
            self_string.push_str("None ");
        }

        if let Some(x) = &self.right_tree {
            self_string.push_str(&point_to_string(x.point));
            x.pretty_print();
        } else {
            self_string.push_str("None ");
        }

        println!("{}", self_string);
    }

    pub fn print(&self) {
        let x = self.collect_points(true);
        println!("{:?}", x);
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
        test_tree.print();
        assert_eq!(test_tree.point, [0, 0]);
    }

    #[test]
    fn make_small_tree() {
        let test_tree = KDTree::new(&vec![[0, 0], [1, 0], [2, 0]], 0, 2);

        assert_eq!(test_tree.point, [1, 0]);

        match &test_tree.left_tree {
            Some(x) => assert_eq!(x.point, [0, 0]),
            None => {
                test_tree.print();
                assert_eq!(0, 1);
            }
        }

        match &test_tree.right_tree {
            Some(x) => assert_eq!(x.point, [2, 0]),
            None => {
                test_tree.print();
                assert_eq!(0, 1);
            }
        }
    }

    #[test]
    fn test_remove_by_point() {
        let mut test_tree = KDTree::new(&vec![[0, 0], [1, 0], [2, 0]], 0, 2);
        test_tree.remove([2, 0]);

        assert_eq!(test_tree.point, [1, 0]);

        match &test_tree.left_tree {
            Some(x) => assert_eq!(x.point, [0, 0]),
            None => {
                test_tree.print();
                assert_eq!(0, 1);
            }
        }

        match &test_tree.right_tree {
            None => assert_eq!(1, 1),
            Some(x) => {
                test_tree.print();
                assert_eq!(0, 1);
            }
        }
    }
}
