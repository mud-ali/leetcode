use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let s = nums.len();
        let r : HashSet<i32> = nums.into_iter().collect();
        r.len() != s
    }
}