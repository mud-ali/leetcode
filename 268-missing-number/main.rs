impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = ((nums.len() + 1) as f32 * nums.len() as f32 * 0.5) as i32;
        let mut sum_nums : i32 = 0;
        for num in nums {
            sum_nums += num;
        }
        return n - sum_nums;
    }
}