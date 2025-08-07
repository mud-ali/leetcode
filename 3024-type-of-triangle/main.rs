use std::collections::HashSet;

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        if nums[0]>=nums[1]+nums[2] || nums[1] >= nums[0] + nums[2] || nums[2] >= nums[1]+nums[0] {
            return String::from("none");
        }
        let set : HashSet<i32> = nums.into_iter().collect();
        let length = set.len();
        let ret = match length {
            1 => "equilateral",
            2 => "isosceles",
            _ => "scalene"
        };

        String::from(ret)
    }
}