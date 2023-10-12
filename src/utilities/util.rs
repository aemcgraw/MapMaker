pub mod util {
    use crate::utilities::kdtree::KDTree;
    use rand::distributions::{Distribution, Uniform};

    pub fn modu(dividend: u32, divisor: u32) -> u32 {
        return ((dividend % divisor) + divisor) % divisor;
    }

    pub fn quickselect(data: &mut Vec<f64>, k: u32) -> f64 {
        return select(data, 0, (data.len() - 1) as u32, k);
    }

    fn select(list: &mut Vec<f64>, left: u32, right: u32, k: u32) -> f64 {
        if left == right {
            return list[left as usize];
        }

        println!("{}", left);

        let mut pivotindex = ((left + right) / 2) as u32;
        pivotindex = partition(list, left, right, pivotindex);

        if k == pivotindex {
            return list[k as usize];
        } else if k < pivotindex {
            return select(list, left, pivotindex - 1, k);   
        } else {
            return select(list, pivotindex + 1, right, k);
        }
    }

    fn partition(list: &mut Vec<f64>, left: u32, right: u32, k: u32) -> u32 {
        let pivot_value = list[k as usize];

        swap(list, right, k);
        let mut storeindex = left;
   
        let mut index = left;
        while index <= right {
            if list[index as usize] < pivot_value {
                swap(list, storeindex, index);
                storeindex += 1;
            }
            index += 1
        }
        //for x in &list[(left as usize)..(right as usize)] {
        //    if list[x as usize] < pivot_value {
        //        swap(list, storeindex, x);
        //        storeindex += 1;
        //    }
        //}

        swap(list, storeindex, k);
        return storeindex;
    }

    fn swap(list: &mut Vec<f64>, x: u32, y: u32) {
        let xvalue = list[x as usize];
        list[x as usize] = list[y as usize];
        list[y as usize] = xvalue;
    }

    pub fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    pub fn generate_kdtree(num_points: u32, width: u32, height: u32) -> KDTree {
        let sampler = Uniform::new(0, width);
        let sampler2 = Uniform::new(0, height);
        let mut rng = rand::thread_rng();

        let mut points = Vec::new();

        let mut x = 0;
        while x < num_points {
            points.push([sampler.sample(&mut rng), sampler2.sample(&mut rng)]);

            x += 1;
        }

        return KDTree::new(&points, 0, 2);
    }
}
