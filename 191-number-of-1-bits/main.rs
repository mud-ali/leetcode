impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut ct : i32 = 1;
        let mut rem = n;
        while rem > 1 {
            ct += rem % 2;
            rem/=2;
        }
        return ct;
    }
}