use std::cmp;
use std::collections::HashMap;

fn gen_dist(vec: &Vec<i32>) -> HashMap<i32, usize> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for &k in vec {
        if let Some(a) = map.get_mut(&k) {
            *a += 1;
        } else {
            map.insert(k, 1);
        }
    }

    map
}

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let dist1 = gen_dist(&nums1);
    let dist2 = gen_dist(&nums2);

    let mut rv = Vec::new();
    for (&k, &v1) in dist1.iter() {
        match dist2.get(&k) {
            Some(&v2) => {
                for _ in 0..cmp::min(v1, v2) {
                    rv.push(k)
                }
            }
            None => continue,
        }
    }

    rv
}

fn main() {
    assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), [2, 2]);
}
