impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::<i32>::new();
        for n in nums1.iter() {
            let mut flag = 0;
            for n2 in nums2.iter() {
                if *n2==*n {
                    flag = 1;
                }

                if flag==1 && *n2 > *n {
                    ans.push(*n2);
                    flag = 2;
                    break;
                }
            }
            if flag!=2 {
                ans.push(-1);
            }
        }
        return ans;
    }
}