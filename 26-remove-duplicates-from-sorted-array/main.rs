impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        for i in (0..nums.len()-1).rev() {
            if nums[i] == nums[i+1] {
                nums.remove(i);
            }
        }
        nums.len() as i32
    }
}