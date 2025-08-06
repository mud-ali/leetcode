impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut pos = 0;
        let mut neg = 0;
        for n in nums {
            if n > 0 { pos += 1; }
            else if n < 0 {neg += 1; }
        }
        if pos > neg { return pos; }
        neg
    }
}