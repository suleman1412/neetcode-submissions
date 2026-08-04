impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // best ans
        let mut pre = 1;
        let mut post = 1;

        let mut res = vec![1; nums.len()];
        for i in 0..nums.len(){
            res[i] = pre;
            pre *= nums[i];
        }

        for i in (0..nums.len()).rev(){
            res[i] *= post;
            post *= nums[i];
        }
        
        res
    }
}
