use std::collections::HashSet;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut max_size = 0;
        let mut left = 0;
        while left < fruits.len() - max_size {
            let mut size = 1;
            let mut unique_fruits = HashSet::new();
            unique_fruits.insert(fruits[left]);
            loop {
                if left+size >= fruits.len() {
                    break;
                }
                unique_fruits.insert(fruits[left+size]);
                if unique_fruits.len() > 2 {
                    break;
                } else {
                    size += 1;
                }
            }
            if size > max_size {
                max_size = size;
            }
            left += 1;
        }
        return max_size as i32;
    }
}