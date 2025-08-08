impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut i=0;
        while i < nums.len() {
            if nums[i] >= target {
                break;
            }
            if nums[i] <= target {
                i += 1;
            }
        }
        return i as i32;
    }
}